use swc_ecma_parser::{EsSyntax, Syntax, TsSyntax};

use crate::{
    filetype::{filename_is_jsx, filename_is_typescript},
    instrumentation_options::InstrumentationOptions,
};

pub fn syntax_for(filename: &str, options: &InstrumentationOptions) -> Syntax {
    let jsx = options
        .input
        .jsx
        .unwrap_or_else(|| filename_is_jsx(filename));
    let typescript = options
        .input
        .typescript
        .unwrap_or_else(|| filename_is_typescript(filename));

    if typescript {
        Syntax::Typescript(TsSyntax {
            tsx: jsx,
            decorators: true,
            dts: false,
            no_early_errors: false,
            disallow_ambiguous_jsx_like: false,
        })
    } else {
        Syntax::Es(EsSyntax {
            jsx,
            fn_bind: true,
            decorators: true,
            decorators_before_export: true,
            export_default_from: true,
            import_attributes: true,
            allow_super_outside_method: true,
            allow_return_outside_function: true,
            auto_accessors: true,
            explicit_resource_management: true,
        })
    }
}
