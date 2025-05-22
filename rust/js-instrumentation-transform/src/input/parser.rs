use swc_ecma_ast::EsVersion::EsNext;
use swc_ecma_parser::{lexer::Lexer, EsSyntax, Parser, StringInput, Syntax, TsSyntax};

use js_instrumentation_shared::{
    filetype::{filename_is_jsx, filename_is_typescript},
    transform_options::TransformOptions,
};

use crate::input::InputFile;

pub fn build_parser<'a>(file: &InputFile<'a>, options: &TransformOptions) -> Parser<Lexer<'a>> {
    let lexer = build_lexer(file, options);
    Parser::new_from(lexer)
}

fn build_lexer<'a>(file: &InputFile<'a>, options: &TransformOptions) -> Lexer<'a> {
    let jsx = options.jsx.unwrap_or_else(|| filename_is_jsx(file.name));
    let typescript = options
        .typescript
        .unwrap_or_else(|| filename_is_typescript(file.name));

    let syntax = if typescript {
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
    };

    Lexer::new(
        syntax,
        EsNext,
        StringInput::new(file.code, file.start_pos, file.end_pos),
        None,
    )
}
