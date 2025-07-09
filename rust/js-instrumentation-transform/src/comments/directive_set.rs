use swc_common::Span;

pub struct DirectiveSet {
    pub privacy_allowlist_excluded_file: bool,
    pub privacy_allowlist_excluded_spans: Vec<Span>,
}

impl DirectiveSet {
    pub fn excludes_span_from_privacy_allowlist(self: &Self, span: &Span) -> bool {
        if self.privacy_allowlist_excluded_file {
            return true;
        }
        for excluded_span in &self.privacy_allowlist_excluded_spans {
            if spans_intersect(span, &excluded_span) {
                return true;
            }
        }
        return false;
    }
}

fn spans_intersect(a: &Span, b: &Span) -> bool {
    if a.hi <= b.lo {
        return false;
    }
    if b.hi <= a.lo {
        return false;
    }
    return true;
}
