use html_escape::decode_html_entities;
use js_instrumentation_shared::log::debug_log;
use lazy_static::lazy_static;
use ordermap::OrderMap;
use regex::Regex;
use swc_atoms::Atom;
use swc_common::{BytePos, Span};

use crate::comments::DirectiveSet;

pub type Dictionary = OrderMap<DictionaryEntry, DictionaryEntryStats>;

const MAX_STRING_LENGTH: usize = 4096;

lazy_static! {
    static ref JSX_INITIAL_WHITESPACE_REGEX: Regex = Regex::new(r"^\n\s+").unwrap();
    static ref JSX_INTERNAL_WHITESPACE_REGEX: Regex = Regex::new(r"\n\s+").unwrap();
    static ref JSX_TERMINAL_WHITESPACE_REGEX: Regex = Regex::new(r"\n\s+$").unwrap();

    static ref JSX_DOUBLE_QUOTE_ATTR_ESCAPED_CHARACTERS_REGEX: Regex = Regex::new(r#"[\\"]"#).unwrap();
    static ref JSX_SINGLE_QUOTE_ATTR_ESCAPED_CHARACTERS_REGEX: Regex = Regex::new(r#"[\\']"#).unwrap();
    static ref JSX_TEXT_ESCAPED_CHARACTERS_REGEX: Regex = Regex::new(r#"[\\"]"#).unwrap();

    /// Matches strings that look like URLs.
    static ref URL_STRINGS_REGEX: Regex =
        Regex::new(r"^(?:http:|https:|data:|url\(|\/\/)").unwrap();

    /// Matches strings that look like file names.
    static ref FILE_NAME_REGEX: Regex =
        Regex::new(r"\.(png|jpe?g|gif|svg|webp|js|cjs|mjs|ts|cts|mts)$").unwrap();

    /// Matches strings that consist only of numbers, or of numbers in [brackets], or of groups of
    /// numbers separated by spaces or dashes or commas or periods.
    static ref NUMERIC_STRINGS_REGEX: Regex = Regex::new(r"^(?:\[?[0-9]+\]?(?:\s|-|,|.)*)+$").unwrap();

    /// Matches strings that look like programming code.
    static ref CODE_STRINGS_REGEX: Regex = Regex::new(r#""use strict""#).unwrap();
}

#[derive(Debug)]
pub struct DictionaryEntryStats {
    pub count: usize,
    pub dictionary_entry: usize,
    pub first_pos: BytePos,
    pub index: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DictionaryEntry {
    String(Atom),
    TaggedTemplate(Vec<Atom>),
    TemplateQuasi(Atom),
}

pub struct DictionaryTracker {
    directive_set: DirectiveSet,
    in_uncollected_scopes: usize,
    pub strings: Dictionary,
}

impl DictionaryTracker {
    pub fn new(directive_set: DirectiveSet) -> DictionaryTracker {
        DictionaryTracker {
            directive_set,
            in_uncollected_scopes: 0,
            strings: Dictionary::default(),
        }
    }

    pub fn enter_uncollected_scope(self: &mut Self) {
        self.in_uncollected_scopes += 1;
    }

    pub fn exit_uncollected_scope(self: &mut Self) {
        if self.in_uncollected_scopes == 0 {
            debug_log("exit_uncollected_scope called outside any uncollected scope.");
            return;
        }
        self.in_uncollected_scopes -= 1;
    }

    pub fn maybe_add_jsx_attribute(
        self: &mut Self,
        raw: &Option<Atom>,
        value: &Atom,
        span: &Span,
    ) -> Option<usize> {
        if self.should_skip_string(value, span) {
            return None;
        }

        let raw_value = match raw {
            Some(raw) => raw,
            None => {
                return None;
            }
        };

        // Escape any real newlines that may be present. JSX attribute values can contain real
        // newlines, but ordinary, non-template JS strings cannot.
        let string = value.replace("\n", "\\n");

        // Decode any HTML entities that appear in the string. Note that it's important that we do
        // this before escaping quotes, since decoding HTML entities can produce new double quotes!
        let string = decode_html_entities(&string);

        if raw_value.starts_with("'") {
            let string =
                JSX_SINGLE_QUOTE_ATTR_ESCAPED_CHARACTERS_REGEX.replace_all(&string, "\\$0");
            let string = format!(r#"'{}'"#, string);
            self.try_add_string(&Some(string.into()), span)
        } else {
            let string =
                JSX_DOUBLE_QUOTE_ATTR_ESCAPED_CHARACTERS_REGEX.replace_all(&string, "\\$0");
            let string = format!(r#""{}""#, string);
            self.try_add_string(&Some(string.into()), span)
        }
    }

    pub fn maybe_add_jsx_text(
        self: &mut Self,
        raw: &Atom,
        value: &Atom,
        span: &Span,
    ) -> Option<usize> {
        if self.should_skip_string(value, span) {
            return None;
        }

        // Collapse whitespace after newlines, consistent with JSX rules.
        let string = JSX_INITIAL_WHITESPACE_REGEX.replace(raw, "");
        let string = JSX_TERMINAL_WHITESPACE_REGEX.replace(&string, "");
        let string = JSX_INTERNAL_WHITESPACE_REGEX.replace_all(&string, " ");

        // Remove any newlines that remain.
        let string = string.replace("\n", "");

        // Decode any HTML entities that appear in the string. Note that it's important that we do
        // this before escaping quotes, since decoding HTML entities can produce new double quotes!
        let string = decode_html_entities(&string);

        // Escape special characters, including double quotes, so that we can safely wrap the
        // string in double quotes.
        let string = JSX_TEXT_ESCAPED_CHARACTERS_REGEX.replace_all(&string, "\\$0");

        // Wrap the string in double quotes.
        let string = format!(r#""{}""#, string);

        Some(self.add_atom(DictionaryEntry::String(string.into()), span))
    }

    pub fn maybe_add_string(
        self: &mut Self,
        raw: &Option<Atom>,
        value: &Atom,
        span: &Span,
    ) -> Option<usize> {
        if self.should_skip_string(value, span) {
            return None;
        }

        self.try_add_string(raw, span)
    }

    fn try_add_string(self: &mut Self, raw: &Option<Atom>, span: &Span) -> Option<usize> {
        match raw {
            Some(raw_value) => {
                return Some(self.add_atom(DictionaryEntry::String(raw_value.clone()), span));
            }
            None => {
                return None;
            }
        }
    }

    pub fn maybe_add_tagged_template(
        self: &mut Self,
        quasis: &Vec<Atom>,
        span: &Span,
    ) -> Option<usize> {
        if self.should_skip_tagged_template(span) {
            return None;
        } else {
            return Some(self.add_atom(DictionaryEntry::TaggedTemplate(quasis.clone()), span));
        }
    }

    pub fn maybe_add_template_quasi(self: &mut Self, raw: &Atom, span: &Span) -> Option<usize> {
        if self.should_skip_string(raw, span) {
            None
        } else {
            Some(self.add_atom(DictionaryEntry::TemplateQuasi(raw.clone()), span))
        }
    }

    fn add_atom(self: &mut Self, atom: DictionaryEntry, span: &Span) -> usize {
        match self.strings.get_mut(&atom) {
            Some(stats) => {
                stats.count += 1;
                stats.index
            }
            None => {
                let index = self.strings.len();
                self.strings.insert(
                    atom,
                    DictionaryEntryStats {
                        count: 1,
                        dictionary_entry: 0,
                        first_pos: span.lo,
                        index,
                    },
                );
                index
            }
        }
    }

    // Ignore empty or otherwise unwanted strings. It's important to use 'value' for this check to
    // ensure that we ignore the surrounding quotes when calculating the length of the string;
    // don't pass 'raw' to this function.
    fn should_skip_string(self: &Self, value: &str, span: &Span) -> bool {
        if self.in_uncollected_scopes > 0 {
            return true;
        }
        if value.len() > MAX_STRING_LENGTH {
            return true;
        }
        if value.trim().len() == 0 {
            return true;
        }
        if self
            .directive_set
            .excludes_span_from_privacy_allowlist(span)
        {
            return true;
        }
        if URL_STRINGS_REGEX.is_match(value) {
            return true;
        }
        if FILE_NAME_REGEX.is_match(value) {
            return true;
        }
        if NUMERIC_STRINGS_REGEX.is_match(value) {
            return true;
        }
        if CODE_STRINGS_REGEX.is_match(value) {
            return true;
        }
        return false;
    }

    fn should_skip_tagged_template(self: &Self, span: &Span) -> bool {
        if self.in_uncollected_scopes > 0 {
            return true;
        }
        if self
            .directive_set
            .excludes_span_from_privacy_allowlist(span)
        {
            return true;
        }
        return false;
    }
}

const ZERO_BENEFIT: usize = 0;
const SINGLE_USE_TAGGED_TEMPLATE_BENEFIT: usize = 1;
const MULTI_USE_TAGGED_TEMPLATE_BENEFIT: usize = 2;
const NON_TAGGED_TEMPLATE_BASE_BENEFIT: usize = 3;

impl DictionaryEntry {
    pub fn max_dict_ref_benefit(self: &Self, dictionary_identifier: &str, count: usize) -> usize {
        match self {
            DictionaryEntry::String(string) => {
                // Dictionary reference: D[<entry>]
                let min_dict_ref_len = dictionary_identifier.len() + /*[*/ 1 + /*_*/ 1 + /*]*/ 1;

                // Original: "<string>"
                let original_len = string.len();

                if min_dict_ref_len < original_len {
                    NON_TAGGED_TEMPLATE_BASE_BENEFIT + (original_len - min_dict_ref_len) * count
                } else {
                    ZERO_BENEFIT
                }
            }
            DictionaryEntry::TaggedTemplate(_) => {
                // Dictionary reference: <tag>(D[<entry>],expr1,expr2)
                // Original: <tag>`quasi1${expr1}quasi2${expr2}`
                // Note that because quasis are totally excluded and expressions require
                // only one separator character instead of three, it's actually quite
                // hard to lose with a tagged template substitution. If we accurately
                // calculated their benefit, they'd usually be bumped to the top of
                // the dictionary, above everything else, but that could be counter-productive
                // since it might push more marginal strings and quasis down the list.
                // For now, we use a much simpler approach that greatly underestimates their
                // benefit.
                if count > 1 {
                    MULTI_USE_TAGGED_TEMPLATE_BENEFIT
                } else {
                    SINGLE_USE_TAGGED_TEMPLATE_BENEFIT
                }
            }
            DictionaryEntry::TemplateQuasi(quasi) => {
                // Dictionary reference: ${D[<entry>]}
                let min_dict_ref_len =
                  /*${*/ 2 + dictionary_identifier.len() + /*[*/ 1 + /*_*/ 1 + /*]}*/ 2;

                // Original: <string>
                let original_len = quasi.len();

                if min_dict_ref_len < original_len {
                    NON_TAGGED_TEMPLATE_BASE_BENEFIT + (original_len - min_dict_ref_len) * count
                } else {
                    ZERO_BENEFIT
                }
            }
        }
    }
}
