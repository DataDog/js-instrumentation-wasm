use swc_common::Span;

use js_instrumentation_shared::log::debug_log;

#[derive(Debug, PartialEq)]
pub enum MarkTarget {
    TaggedTemplateBeforeExpr,
    TaggedTemplateAfterExpr,
    TaggedTemplateTerminator,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReplaceTarget {
    JSXString,
    PropertyKey,
    String,
    TaggedTemplateOpener,
    TemplateQuasi,
}

#[derive(Debug, PartialEq)]
pub enum Rewrite {
    Mark {
        target: MarkTarget,
        span: Span,
    },
    Replace {
        target: ReplaceTarget,
        dictionary_index: usize,
        span: Span,
    },
}

impl Rewrite {
    pub fn jsx_string(dictionary_index: usize, span: Span) -> Rewrite {
        Rewrite::Replace {
            target: ReplaceTarget::JSXString,
            dictionary_index,
            span,
        }
    }

    pub fn property_key(dictionary_index: usize, span: Span) -> Rewrite {
        Rewrite::Replace {
            target: ReplaceTarget::PropertyKey,
            dictionary_index,
            span,
        }
    }

    pub fn string(dictionary_index: usize, span: Span) -> Rewrite {
        Rewrite::Replace {
            target: ReplaceTarget::String,
            dictionary_index,
            span,
        }
    }

    pub fn tagged_template_opener(dictionary_index: usize, span: Span) -> Rewrite {
        Rewrite::Replace {
            target: ReplaceTarget::TaggedTemplateOpener,
            dictionary_index,
            span,
        }
    }

    pub fn tagged_template_before_expr(span: Span) -> Rewrite {
        Rewrite::Mark {
            target: MarkTarget::TaggedTemplateBeforeExpr,
            span,
        }
    }

    pub fn tagged_template_after_expr(span: Span) -> Rewrite {
        Rewrite::Mark {
            target: MarkTarget::TaggedTemplateAfterExpr,
            span,
        }
    }

    pub fn tagged_template_terminator(span: Span) -> Rewrite {
        Rewrite::Mark {
            target: MarkTarget::TaggedTemplateTerminator,
            span,
        }
    }

    pub fn template_quasi(dictionary_index: usize, span: Span) -> Rewrite {
        Rewrite::Replace {
            target: ReplaceTarget::TemplateQuasi,
            dictionary_index,
            span,
        }
    }
}

pub struct RewriteTracker {
    pub rewrites: Vec<Rewrite>,
    in_unrewritten_scopes: usize,
}

impl RewriteTracker {
    pub fn new() -> RewriteTracker {
        RewriteTracker {
            rewrites: vec![],
            in_unrewritten_scopes: 0,
        }
    }

    pub fn enter_unrewritten_scope(self: &mut Self) {
        self.in_unrewritten_scopes += 1;
    }

    pub fn exit_unrewritten_scope(self: &mut Self) {
        if self.in_unrewritten_scopes == 0 {
            debug_log("exit_unrewritten_scope called outside any uncollected scope.");
            return;
        }
        self.in_unrewritten_scopes -= 1;
    }

    pub fn emit(self: &mut Self, span: Rewrite) {
        if self.in_unrewritten_scopes > 0 {
            return;
        }

        self.rewrites.push(span);
    }
}
