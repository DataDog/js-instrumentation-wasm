use crate::rewrite::{MarkTarget, ReplaceTarget, Rewrite};

use super::{EmitContext, EmitError, OutputEmitter, OutputScopeKind};

pub fn emit_rewritten_input(
    ctx: &EmitContext,
    output: &mut OutputEmitter,
    rewrites: &Vec<Rewrite>,
) {
    for rewrite in rewrites {
        match rewrite {
            Rewrite::Mark { target, span } => match target {
                MarkTarget::TaggedTemplateBeforeExpr => {
                    output.emit_replacement_until(span.hi, ", ");
                    output.enter_scope(OutputScopeKind::Valid);
                }
                MarkTarget::TaggedTemplateAfterExpr => {
                    output.exit_scope();
                    output.emit_input_until(span.hi);
                }
                MarkTarget::TaggedTemplateTerminator => {
                    output.emit_replacement_until(span.hi, ")");
                    output.exit_scope();
                }
            },
            Rewrite::Replace {
                target,
                dictionary_index,
                span,
            } => match ctx.dictionary.strings.get_index(*dictionary_index) {
                Some((_, ref stats)) => match target {
                    ReplaceTarget::JSXString => {
                        output.emit_input_until(span.lo);
                        output.emit_input_or_replacement_until(
                            span.hi,
                            &jsx_string_dictionary_reference(ctx, stats.dictionary_entry),
                        );
                    }
                    ReplaceTarget::PropertyKey => {
                        output.emit_input_until(span.lo);
                        output.emit_input_or_replacement_until(
                            span.hi,
                            &property_key_dictionary_reference(ctx, stats.dictionary_entry),
                        );
                    }
                    ReplaceTarget::String => {
                        output.emit_input_until(span.lo);
                        output.emit_input_or_replacement_until(
                            span.hi,
                            &string_dictionary_reference(ctx, stats.dictionary_entry),
                        );
                    }
                    ReplaceTarget::TaggedTemplateOpener => {
                        output.enter_scope(OutputScopeKind::Valid);
                        output.emit_input_until(span.lo);
                        output.emit_replacement_until(
                            span.hi,
                            &template_opener_dictionary_reference(ctx, stats.dictionary_entry),
                        );
                    }
                    ReplaceTarget::TemplateQuasi => {
                        output.emit_input_until(span.lo);
                        output.emit_input_or_replacement_until(
                            span.hi,
                            &template_quasi_dictionary_reference(ctx, stats.dictionary_entry),
                        );
                    }
                },
                None => {
                    if *target == ReplaceTarget::TaggedTemplateOpener {
                        output.enter_scope(OutputScopeKind::Invalid);
                    }
                    output.emit_input_until(span.hi);
                    output.report_error(EmitError::InvalidStringIndex(
                        target.clone(),
                        *dictionary_index,
                    ));
                }
            },
        }
    }

    output.emit_rest_of_input();
}

fn jsx_string_dictionary_reference(ctx: &EmitContext, index: usize) -> String {
    format!("{{{}[{}]}}", ctx.dictionary_identifier, index)
}

fn property_key_dictionary_reference(ctx: &EmitContext, index: usize) -> String {
    format!("[{}[{}]]", ctx.dictionary_identifier, index)
}

fn string_dictionary_reference(ctx: &EmitContext, index: usize) -> String {
    format!("{}[{}]", ctx.dictionary_identifier, index)
}

fn template_opener_dictionary_reference(ctx: &EmitContext, index: usize) -> String {
    format!("({}[{}]", ctx.dictionary_identifier, index)
}

fn template_quasi_dictionary_reference(ctx: &EmitContext, index: usize) -> String {
    format!("${{{}[{}]}}", ctx.dictionary_identifier, index)
}
