use js_instrumentation_shared::module_kind::ModuleKind;

use crate::dictionary::DictionaryEntry;

use super::{EmitContext, OutputEmitter};

pub fn emit_banner(ctx: &EmitContext, output: &mut OutputEmitter) {
    // Don't generate the preamble if we didn't collect any strings.
    if ctx.dictionary.strings.is_empty() {
        return;
    }

    match ctx.module_kind {
        ModuleKind::CJS if ctx.helper_identifier == "$" => {
            output.emit(&format!(
                "const {{ $ }} = require('{}');\n",
                ctx.helpers_module
            ));
        }
        ModuleKind::CJS => {
            output.emit(&format!(
                "const {{ $: {} }} = require('{}');\n",
                ctx.helper_identifier, ctx.helpers_module,
            ));
        }
        ModuleKind::ESM if ctx.helper_identifier == "$" => {
            output.emit(&format!("import {{ $ }} from '{}';\n", ctx.helpers_module));
        }
        ModuleKind::ESM => {
            output.emit(&format!(
                "import {{ $ as {} }} from '{}';\n",
                ctx.helper_identifier, ctx.helpers_module
            ));
        }
    }

    output.emit(&format!(
        "const {} = {}([\n",
        ctx.dictionary_identifier, ctx.helper_identifier
    ));

    for index in &ctx.dictionary.indices {
        match ctx.dictionary.strings.get_index(*index) {
            Some((atom, _stats)) => {
                output.emit("  ");
                match atom {
                    DictionaryEntry::String(string) => {
                        output.emit(string.as_str());
                    }
                    DictionaryEntry::TaggedTemplate(quasis) => {
                        output.emit(&format!("{}`", ctx.helper_identifier));
                        let mut need_separator = false;
                        for quasi in quasis {
                            if need_separator {
                                output.emit("${0}");
                            } else {
                                need_separator = true;
                            }
                            output.emit(quasi.as_str());
                        }
                        output.emit("`");
                    }
                    DictionaryEntry::TemplateQuasi(quasi) => {
                        output.emit("`");
                        output.emit(quasi.as_str());
                        output.emit("`");
                    }
                }
                output.emit(",\n");
            }
            None => {}
        }
    }

    output.emit("]);\n");
}
