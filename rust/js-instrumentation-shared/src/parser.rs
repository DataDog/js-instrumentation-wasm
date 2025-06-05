use swc_ecma_ast::EsVersion::EsNext;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput};

use crate::{input_file::InputFile, syntax_for, transform_options::TransformOptions};

pub fn build_parser<'a>(file: &InputFile<'a>, options: &TransformOptions) -> Parser<Lexer<'a>> {
    let lexer = build_lexer(file, options);
    Parser::new_from(lexer)
}

fn build_lexer<'a>(file: &InputFile<'a>, options: &TransformOptions) -> Lexer<'a> {
    let syntax = syntax_for(file.name, options);
    Lexer::new(
        syntax,
        EsNext,
        StringInput::new(file.code, file.start_pos, file.end_pos),
        None,
    )
}
