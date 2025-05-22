use crate::{input::InputFile, rewrite::Rewrite};

use super::{emit_banner, emit_rewritten_input, EmitContext, EmitError, OutputEmitter};

pub fn emit<'a>(
    ctx: EmitContext,
    input_file: &InputFile<'a>,
    rewrites: &Vec<Rewrite>,
) -> (String, Vec<EmitError>) {
    let mut output = OutputEmitter::new(input_file);
    emit_banner(&ctx, &mut output);
    emit_rewritten_input(&ctx, &mut output, rewrites);
    return output.take();
}
