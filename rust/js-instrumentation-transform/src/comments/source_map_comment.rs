use swc_common::Span;

pub enum SourceMapComment {
    Inline(Vec<u8>, Span),
    External(),
}
