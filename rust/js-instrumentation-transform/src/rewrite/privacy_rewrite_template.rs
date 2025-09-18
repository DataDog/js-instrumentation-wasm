use std::fmt::{Display, Write};

use js_instrumentation_shared::{instrumentation_options::HelperFunctionSource, ModuleKind};

use crate::dictionary::{DictionaryEntry, DictionaryError, OptimizedDictionary};

use super::PrivacyRewriteContent;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LeftContext {
    MaybeKeyword,
    NonKeyword,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrivacyRewriteTemplate {
    HelperDeclaration,
    DictionaryDeclaration,
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
            PrivacyRewriteTemplate::HelperDeclaration => Ok(PrivacyRewriteContent::HelperImport(
                build_helper_declaration(params),
            )),
            PrivacyRewriteTemplate::DictionaryDeclaration => Ok(
                PrivacyRewriteContent::DictionaryDeclaration(build_dictionary_declaration(params)),
            ),
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

fn build_helper_declaration(params: &TemplateParameters) -> String {
    // Don't generate an import statement if we didn't collect any strings.
    if params.dictionary.strings.is_empty() {
        return "".to_string();
    }

    match &params.add_to_dictionary_helper_source {
        HelperFunctionSource::Expression { code } => format!(
            "const {}={};",
            params.add_to_dictionary_helper_identifier, code
        ),
        HelperFunctionSource::Import {
            cjs_module,
            esm_module,
            func,
        } => match params.module_kind {
            ModuleKind::CJS if &params.add_to_dictionary_helper_identifier == func => {
                format!(
                    "const{{{}}}=require('{}');",
                    params.add_to_dictionary_helper_identifier, cjs_module,
                )
            }
            ModuleKind::CJS => {
                format!(
                    "const{{{}:{}}}=require('{}');",
                    func, params.add_to_dictionary_helper_identifier, cjs_module,
                )
            }
            ModuleKind::ESM if &params.add_to_dictionary_helper_identifier == func => {
                format!(
                    "import{{{}}}from'{}';",
                    params.add_to_dictionary_helper_identifier, esm_module,
                )
            }
            ModuleKind::ESM => {
                format!(
                    "import{{{} as {}}}from'{}';",
                    func, params.add_to_dictionary_helper_identifier, esm_module,
                )
            }
        },
    }
}

fn build_dictionary_declaration(params: &TemplateParameters) -> String {
    // Don't generate a dictionary declaration if we didn't collect any strings.
    if params.dictionary.strings.is_empty() {
        return "".to_string();
    }

    let mut output = String::new();

    let _ = write!(
        &mut output,
        "const {}={}([",
        params.dictionary_identifier, params.add_to_dictionary_helper_identifier
    );

    let mut follows_another_entry = false;
    for index in &params.dictionary.indices {
        match params.dictionary.strings.get_index(*index) {
            Some((atom, _stats)) => {
                if follows_another_entry {
                    let _ = write!(&mut output, "{}", ",");
                }

                match atom {
                    DictionaryEntry::String(string) => {
                        let _ = write!(&mut output, "{}", string);
                    }
                    DictionaryEntry::TaggedTemplate(quasis) => {
                        let _ = write!(
                            &mut output,
                            "{}`",
                            params.add_to_dictionary_helper_identifier
                        );
                        let mut need_separator = false;
                        for quasi in quasis {
                            if need_separator {
                                let _ = write!(&mut output, "{}", "${0}");
                            } else {
                                need_separator = true;
                            }
                            let _ = write!(&mut output, "{}", quasi.as_str());
                        }
                        let _ = write!(&mut output, "{}", "`");
                    }
                    DictionaryEntry::TemplateQuasi(quasi) => {
                        let _ = write!(&mut output, "`{}`", quasi.as_str());
                    }
                }

                follows_another_entry = true;
            }
            None => {}
        }
    }

    let _ = write!(&mut output, "{}", "]);");

    return output;
}
