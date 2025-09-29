use js_instrumentation_rewrite::rewrite::Rewrite;
use swc_common::Span;

use super::{privacy_rewrite_template::LeftContext, PrivacyRewriteTemplate};

pub fn replace_tagged_template_opener_with_dictionary_ref(
    dictionary_index: usize,
    span: Span,
) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::TaggedTemplateOpenerDictionaryReference(dictionary_index),
        span,
    }
}

pub fn replace_tagged_template_before_expr_marker(span: Span) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::TaggedTemplateBeforeExpr,
        span,
    }
}

pub fn replace_tagged_template_after_expr_marker(span: Span) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::TaggedTemplateAfterExpr,
        span,
    }
}

pub fn replace_tagged_template_terminator(span: Span) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::TaggedTemplateTerminator,
        span,
    }
}

pub fn replace_jsx_string_with_dictionary_ref(
    dictionary_index: usize,
    span: Span,
) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::JSXStringDictionaryReference(dictionary_index),
        span,
    }
}

pub fn replace_property_key_with_dictionary_ref(
    dictionary_index: usize,
    span: Span,
) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::PropertyKeyDictionaryReference(dictionary_index),
        span,
    }
}

pub fn replace_string_with_dictionary_ref(
    dictionary_index: usize,
    span: Span,
    may_follow_keyword: bool,
) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::StringDictionaryReference(
            dictionary_index,
            if may_follow_keyword {
                LeftContext::MaybeKeyword
            } else {
                LeftContext::NonKeyword
            },
        ),
        span,
    }
}

pub fn replace_template_quasi_with_dictionary_ref(
    dictionary_index: usize,
    span: Span,
) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::TemplateQuasiDictionaryReference(dictionary_index),
        span,
    }
}

pub fn delete_source_map_comment(span: Span) -> Rewrite<PrivacyRewriteTemplate> {
    Rewrite::Replace {
        content: PrivacyRewriteTemplate::DeleteSourceMapComment,
        span,
    }
}
