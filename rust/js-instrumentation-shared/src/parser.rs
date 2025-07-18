use swc_common::comments::SingleThreadedComments;
use swc_ecma_ast::EsVersion::EsNext;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput};

use crate::{input_file::InputFile, instrumentation_options::InstrumentationOptions, syntax_for};

pub fn build_parser<'a>(
    file: &InputFile<'a>,
    comments: &'a SingleThreadedComments,
    options: &InstrumentationOptions,
) -> Parser<Lexer<'a>> {
    let lexer = build_lexer(file, comments, options);
    Parser::new_from(lexer)
}

fn build_lexer<'a>(
    file: &InputFile<'a>,
    comments: &'a SingleThreadedComments,
    options: &InstrumentationOptions,
) -> Lexer<'a> {
    let syntax = syntax_for(file.name, options);
    Lexer::new(
        syntax,
        EsNext,
        StringInput::new(file.code, file.start_pos, file.end_pos),
        Some(comments),
    )
}
