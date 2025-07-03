use js_instrumentation_shared::log::debug_log;
use lazy_static::lazy_static;
use ordermap::OrderMap;
use regex::Regex;
use swc_atoms::Atom;

pub type Dictionary = OrderMap<DictionaryEntry, DictionaryEntryStats>;

lazy_static! {
    static ref JSX_INITIAL_WHITESPACE_REGEX: Regex = Regex::new(r"^\n\s+").unwrap();
    static ref JSX_INTERNAL_WHITESPACE_REGEX: Regex = Regex::new(r"\n\s+").unwrap();
    static ref JSX_TERMINAL_WHITESPACE_REGEX: Regex = Regex::new(r"\n\s+$").unwrap();
    static ref JSX_ESCAPED_CHARACTERS_REGEX: Regex = Regex::new(r#"[\\"]"#).unwrap();

    /// Matches strings that look like URLs.
    static ref URL_STRINGS_REGEX: Regex =
        Regex::new(r"^(?:http:|https:|data:|url\(|\/\/)").unwrap();

    /// Matches strings that look like file names.
    static ref FILE_NAME_REGEX: Regex =
        Regex::new(r"\.(png|jpe?g|gif|svg|webp|js|cjs|mjs|ts|cts|mts)$").unwrap();

    /// Matches strings that consist only of numbers, or of numbers in [brackets], or of groups of
    /// numbers separated by spaces or dashes or commas or periods.
    static ref NUMERIC_STRINGS_REGEX: Regex = Regex::new(r"^(?:\[?[0-9]+\]?(?:\s|-|,|.)*)+$").unwrap();
}

pub struct DictionaryEntryStats {
    pub count: usize,
    pub dictionary_entry: usize,
    pub index: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DictionaryEntry {
    String(Atom),
    TaggedTemplate(Vec<Atom>),
    TemplateQuasi(Atom),
}

pub struct DictionaryTracker {
    pub strings: Dictionary,
    in_uncollected_scopes: usize,
}

impl DictionaryTracker {
    pub fn new() -> DictionaryTracker {
        DictionaryTracker {
            strings: Dictionary::default(),
            in_uncollected_scopes: 0,
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
    ) -> Option<usize> {
        if self.should_skip_string(value) {
            return None;
        }

        let raw_value = match raw {
            Some(raw) => raw,
            None => {
                return None;
            }
        };

        // The input to this function is a JSX attribute value, which may be either a normal JS
        // string (if it was surrounded by '{}') or a JSX string with HTML-like behavior (if it
        // wasn't). Unfortunately, SWC's lexer does not make the braces around JSX attribute values
        // visible to us, so we can't tell which situation we're in. So, this code needs to be
        // written in such a way as to handle both normal JS strings and JSX strings. Fortunately,
        // because SWC already does some normalization on the token's 'value' before handing it to
        // us, they can mostly be treated uniformly, but we do need to be careful to deal with some
        // oddities specific to JSX strings, like the potential for real newlines to be present.
        let string = value.replace("\n", "\\n");

        if raw_value.starts_with("\"") {
            let string = format!(r#""{}""#, string);
            self.maybe_add_string(&Some(string.into()), value)
        } else if raw_value.starts_with("'") {
            let string = format!(r#"'{}'"#, string);
            self.maybe_add_string(&Some(string.into()), value)
        } else {
            let string = string.replace("\"", "\\\"");
            let string = format!(r#""{}""#, string);
            self.maybe_add_string(&Some(string.into()), value)
        }
    }

    pub fn maybe_add_jsx_text(self: &mut Self, raw: &Atom, value: &Atom) -> Option<usize> {
        if self.should_skip_string(value) {
            return None;
        }

        // Collapse whitespace after newlines, consistent with JSX rules.
        let string = JSX_INITIAL_WHITESPACE_REGEX.replace(raw, "");
        let string = JSX_TERMINAL_WHITESPACE_REGEX.replace(&string, "");
        let string = JSX_INTERNAL_WHITESPACE_REGEX.replace_all(&string, " ");

        // Remove any newlines that remain.
        let string = string.replace("\n", "");

        // Escape double quotes, so that we can safely wrap the string in double
        // quotes.
        let string = JSX_ESCAPED_CHARACTERS_REGEX.replace_all(&string, "\\$0");

        // Wrap the string in double quotes.
        let string = format!(r#""{}""#, string);

        Some(self.add_atom(DictionaryEntry::String(string.into())))
    }

    pub fn maybe_add_string(self: &mut Self, raw: &Option<Atom>, value: &Atom) -> Option<usize> {
        if self.should_skip_string(value) {
            return None;
        }

        match raw {
            Some(raw_value) => {
                return Some(self.add_atom(DictionaryEntry::String(raw_value.clone())));
            }
            None => {
                return None;
            }
        }
    }

    pub fn maybe_add_tagged_template(self: &mut Self, quasis: &Vec<Atom>) -> Option<usize> {
        if self.should_skip_tagged_template() {
            return None;
        } else {
            return Some(self.add_atom(DictionaryEntry::TaggedTemplate(quasis.clone())));
        }
    }

    pub fn maybe_add_template_quasi(self: &mut Self, raw: &Atom) -> Option<usize> {
        if self.should_skip_string(raw) {
            None
        } else {
            Some(self.add_atom(DictionaryEntry::TemplateQuasi(raw.clone())))
        }
    }

    fn add_atom(self: &mut Self, atom: DictionaryEntry) -> usize {
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
    fn should_skip_string(self: &Self, value: &str) -> bool {
        if self.in_uncollected_scopes > 0 {
            return true;
        }
        if value.trim().len() == 0 {
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
        return false;
    }

    fn should_skip_tagged_template(self: &Self) -> bool {
        if self.in_uncollected_scopes > 0 {
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
