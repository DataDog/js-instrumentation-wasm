use std::fmt::Display;

use js_instrumentation_rewrite::rewrite_content::RewriteContent;
use js_instrumentation_shared::{instrumentation_options::HelperFunctionSource, ModuleKind};
use swc_common::BytePos;

use crate::dictionary::{DictionaryError, OptimizedDictionary};

use super::PrivacyRewriteContent;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LeftContext {
    MaybeKeyword,
    NonKeyword,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrivacyRewriteTemplate {
    JSXStringDictionaryReference(usize),
    PropertyKeyDictionaryReference(usize),
    StringDictionaryReference(usize, LeftContext),
    TaggedTemplateOpenerDictionaryReference(usize),
    TaggedTemplateBeforeExpr,
    TaggedTemplateAfterExpr,
    TaggedTemplateTerminator,
    TemplateQuasiDictionaryReference(usize),
    DeleteSourceMapComment,
}

impl RewriteContent for PrivacyRewriteTemplate {
    fn source_pos(self: &Self) -> Option<BytePos> {
        None
    }
}

pub struct TemplateParameters<'a> {
    pub dictionary: OptimizedDictionary,
    pub dictionary_identifier: String,
    pub add_to_dictionary_helper_source: &'a HelperFunctionSource,
    pub add_to_dictionary_helper_identifier: String,
    pub module_kind: ModuleKind,
}

impl<'a> TemplateParameters<'a> {
    pub fn new(
        dictionary: OptimizedDictionary,
        dictionary_identifier: String,
        add_to_dictionary_helper_source: &HelperFunctionSource,
        add_to_dictionary_helper_identifier: String,
        module_kind: ModuleKind,
    ) -> TemplateParameters<'_> {
        TemplateParameters {
            dictionary,
            dictionary_identifier,
            add_to_dictionary_helper_source,
            add_to_dictionary_helper_identifier,
            module_kind,
        }
    }
}

impl PrivacyRewriteTemplate {
    pub fn evaluate(
        self: &Self,
        params: &TemplateParameters,
    ) -> Result<PrivacyRewriteContent, DictionaryError> {
        match self {
            PrivacyRewriteTemplate::JSXStringDictionaryReference(index) => Ok(
                PrivacyRewriteContent::JSXStringDictionaryReference(format!(
                    "{{{}[{}]}}",
                    params.dictionary_identifier,
                    params.dictionary.entry_for_index(*index)?
                )),
            ),
            PrivacyRewriteTemplate::PropertyKeyDictionaryReference(index) => Ok(
                PrivacyRewriteContent::PropertyKeyDictionaryReference(format!(
                    "[{}[{}]]",
                    params.dictionary_identifier,
                    params.dictionary.entry_for_index(*index)?
                )),
            ),
            PrivacyRewriteTemplate::StringDictionaryReference(index, left_context) => {
                Ok(match left_context {
                    // If the character immediately to the left of the string looks like a keyword,
                    // we have a construction like `case"foo"`. In this situation, we need to
                    // insert an extra space to produce valid code after rewriting; otherwise,
                    // we'd end up with `caseD[0]`, which is invalid.
                    LeftContext::MaybeKeyword => {
                        PrivacyRewriteContent::StringDictionaryReference(format!(
                            " {}[{}]",
                            params.dictionary_identifier,
                            params.dictionary.entry_for_index(*index)?
                        ))
                    }
                    LeftContext::NonKeyword => {
                        PrivacyRewriteContent::StringDictionaryReference(format!(
                            "{}[{}]",
                            params.dictionary_identifier,
                            params.dictionary.entry_for_index(*index)?
                        ))
                    }
                })
            }
            PrivacyRewriteTemplate::TaggedTemplateOpenerDictionaryReference(index) => Ok(
                PrivacyRewriteContent::TaggedTemplateOpenerDictionaryReference(format!(
                    "({}[{}]",
                    params.dictionary_identifier,
                    params.dictionary.entry_for_index(*index)?
                )),
            ),
            PrivacyRewriteTemplate::TaggedTemplateBeforeExpr => Ok(
                PrivacyRewriteContent::TaggedTemplateBeforeExpr(", ".to_string()),
            ),
            PrivacyRewriteTemplate::TaggedTemplateAfterExpr => Ok(
                PrivacyRewriteContent::TaggedTemplateAfterExpr("".to_string()),
            ),
            PrivacyRewriteTemplate::TaggedTemplateTerminator => Ok(
                PrivacyRewriteContent::TaggedTemplateTerminator(")".to_string()),
            ),
            PrivacyRewriteTemplate::TemplateQuasiDictionaryReference(index) => Ok(
                PrivacyRewriteContent::TemplateQuasiDictionaryReference(format!(
                    "${{{}[{}]}}",
                    params.dictionary_identifier,
                    params.dictionary.entry_for_index(*index)?
                )),
            ),
            PrivacyRewriteTemplate::DeleteSourceMapComment => Ok(
                PrivacyRewriteContent::DeleteSourceMapComment("".to_string()),
            ),
        }
    }
}

impl Display for PrivacyRewriteTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // PrivacyRewriteTemplate is an implementation detail that never appears in the final
        // output, so we can just display its derived Debug implementation.
        write!(f, "{:?}", self)
    }
}
