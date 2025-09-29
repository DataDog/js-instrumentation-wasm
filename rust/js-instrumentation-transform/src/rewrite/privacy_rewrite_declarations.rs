use std::fmt::Write;

use js_instrumentation_rewrite::rewrite::Rewrite;
use js_instrumentation_shared::{instrumentation_options::HelperFunctionSource, ModuleKind};
use swc_common::BytePos;

use crate::{dictionary::DictionaryEntry, rewrite::TemplateParameters};

use super::PrivacyRewriteContent;

pub fn build_helper_declaration(
    pos: BytePos,
    params: &TemplateParameters,
) -> Vec<Rewrite<PrivacyRewriteContent>> {
    let mut declaration: Vec<Rewrite<PrivacyRewriteContent>> = Vec::new();

    // Don't generate an import statement if we didn't collect any strings.
    if params.dictionary.strings.is_empty() {
        return declaration;
    }

    match &params.add_to_dictionary_helper_source {
        HelperFunctionSource::Expression { code } => {
            declaration.push(Rewrite::Insert {
                content: PrivacyRewriteContent::HelperImport(format!(
                    "const {}={};",
                    params.add_to_dictionary_helper_identifier, code
                )),
                pos,
            });
        }
        HelperFunctionSource::Import {
            cjs_module,
            esm_module,
            func,
        } => match params.module_kind {
            ModuleKind::CJS if &params.add_to_dictionary_helper_identifier == func => {
                declaration.push(Rewrite::Insert {
                    content: PrivacyRewriteContent::HelperImport(format!(
                        "const{{{}}}=require('{}');",
                        params.add_to_dictionary_helper_identifier, cjs_module,
                    )),
                    pos,
                });
            }
            ModuleKind::CJS => {
                declaration.push(Rewrite::Insert {
                    content: PrivacyRewriteContent::HelperImport(format!(
                        "const{{{}:{}}}=require('{}');",
                        func, params.add_to_dictionary_helper_identifier, cjs_module,
                    )),
                    pos,
                });
            }
            ModuleKind::ESM if &params.add_to_dictionary_helper_identifier == func => {
                declaration.push(Rewrite::Insert {
                    content: PrivacyRewriteContent::HelperImport(format!(
                        "import{{{}}}from'{}';",
                        params.add_to_dictionary_helper_identifier, esm_module,
                    )),
                    pos,
                });
            }
            ModuleKind::ESM => {
                declaration.push(Rewrite::Insert {
                    content: PrivacyRewriteContent::HelperImport(format!(
                        "import{{{} as {}}}from'{}';",
                        func, params.add_to_dictionary_helper_identifier, esm_module,
                    )),
                    pos,
                });
            }
        },
    };

    return declaration;
}

pub fn build_dictionary_declaration(
    pos: BytePos,
    params: &TemplateParameters,
) -> Vec<Rewrite<PrivacyRewriteContent>> {
    let mut declaration: Vec<Rewrite<PrivacyRewriteContent>> = Vec::new();

    // Don't generate a dictionary declaration if we didn't collect any strings.
    if params.dictionary.strings.is_empty() {
        return declaration;
    }

    declaration.push(Rewrite::Insert {
        content: PrivacyRewriteContent::DictionaryDeclarationOpener(format!(
            "const {}={}([",
            params.dictionary_identifier, params.add_to_dictionary_helper_identifier
        )),
        pos,
    });

    let mut follows_another_entry = false;
    for index in &params.dictionary.indices {
        match params.dictionary.strings.get_index(*index) {
            Some((atom, stats)) => {
                let mut output = String::new();

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

                declaration.push(Rewrite::Insert {
                    content: PrivacyRewriteContent::DictionaryDeclarationEntry(
                        output,
                        stats.first_pos,
                    ),
                    pos,
                });
            }
            None => {}
        }
    }

    declaration.push(Rewrite::Insert {
        content: PrivacyRewriteContent::DictionaryDeclarationCloser("]);".to_string()),
        pos,
    });

    return declaration;
}
