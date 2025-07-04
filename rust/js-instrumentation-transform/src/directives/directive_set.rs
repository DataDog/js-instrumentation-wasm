use js_instrumentation_shared::InputFile;
use swc_common::{
    comments::{Comment, SingleThreadedComments},
    BytePos, Span,
};

const PRIVACY_ALLOWLIST_EXCLUDE_BEGIN_COMMENT: &'static str =
    "datadog-privacy-allowlist-exclude-begin";
const PRIVACY_ALLOWLIST_EXCLUDE_END_COMMENT: &'static str = "datadog-privacy-allowlist-exclude-end";
const PRIVACY_ALLOWLIST_EXCLUDE_FILE_COMMENT: &'static str =
    "datadog-privacy-allowlist-exclude-file";
const PRIVACY_ALLOWLIST_EXCLUDE_LINE_COMMENT: &'static str =
    "datadog-privacy-allowlist-exclude-line";
const PRIVACY_ALLOWLIST_EXCLUDE_NEXT_LINE_COMMENT: &'static str =
    "datadog-privacy-allowlist-exclude-next-line";

pub struct DirectiveSet {
    privacy_allowlist_excluded_file: bool,
    privacy_allowlist_excluded_spans: Vec<Span>,
}

impl DirectiveSet {
    pub fn new(file: &InputFile, comments: &SingleThreadedComments) -> Self {
        let mut privacy_allowlist_excluded_file = false;
        let mut privacy_allowlist_excluded_spans: Vec<Span> = Vec::new();
        let mut exclusion_begin_positions: Vec<BytePos> = Vec::new();
        let mut exclusion_end_positions: Vec<BytePos> = Vec::new();

        let (leading_comments, trailing_comments) = comments.borrow_all();
        for (_pos, comments) in leading_comments.iter().chain(trailing_comments.iter()) {
            for comment in comments {
                if let Some(span) = parse_privacy_allowlist_excluded_span_directive(file, comment) {
                    if span == file.span() {
                        privacy_allowlist_excluded_file = true;
                    } else {
                        privacy_allowlist_excluded_spans.push(span);
                    }
                } else if let Some(pos) = parse_privacy_allowlist_exclude_begin_directive(comment) {
                    exclusion_begin_positions.push(pos);
                } else if let Some(pos) = parse_privacy_allowlist_exclude_end_directive(comment) {
                    exclusion_end_positions.push(pos);
                }
            }
        }

        add_spans_for_exclusion_begin_and_end_positions(
            file,
            &mut privacy_allowlist_excluded_spans,
            exclusion_begin_positions,
            exclusion_end_positions,
        );

        return DirectiveSet {
            privacy_allowlist_excluded_file,
            privacy_allowlist_excluded_spans,
        };
    }

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

fn parse_privacy_allowlist_excluded_span_directive(
    file: &InputFile,
    comment: &Comment,
) -> Option<Span> {
    let text = comment.text.as_str().trim();
    if text == PRIVACY_ALLOWLIST_EXCLUDE_FILE_COMMENT {
        Some(Span {
            lo: file.start_pos,
            hi: file.end_pos,
        })
    } else if text == PRIVACY_ALLOWLIST_EXCLUDE_LINE_COMMENT {
        bounds_of_lines_intersecting_span(comment.span, file)
    } else if text == PRIVACY_ALLOWLIST_EXCLUDE_NEXT_LINE_COMMENT {
        bounds_of_line_after_pos(comment.span.hi, file)
    } else {
        None
    }
}

fn parse_privacy_allowlist_exclude_begin_directive(comment: &Comment) -> Option<BytePos> {
    let text = comment.text.as_str().trim();
    if text == PRIVACY_ALLOWLIST_EXCLUDE_BEGIN_COMMENT {
        Some(comment.span.lo)
    } else {
        None
    }
}

fn parse_privacy_allowlist_exclude_end_directive(comment: &Comment) -> Option<BytePos> {
    let text = comment.text.as_str().trim();
    if text == PRIVACY_ALLOWLIST_EXCLUDE_END_COMMENT {
        Some(comment.span.hi)
    } else {
        None
    }
}

fn add_spans_for_exclusion_begin_and_end_positions(
    file: &InputFile,
    privacy_allowlist_excluded_spans: &mut Vec<Span>,
    mut exclusion_begin_positions: Vec<BytePos>,
    mut exclusion_end_positions: Vec<BytePos>,
) {
    exclusion_begin_positions.sort_unstable();
    exclusion_end_positions.sort_unstable();

    // Convert begin/end directives into spans.
    let mut exclusion_end_iter = exclusion_end_positions.into_iter();
    let mut last_end_pos: Option<BytePos> = None;
    for begin_pos in exclusion_begin_positions {
        // If there are other begin directives between a begin directive and the next end
        // directive, ignore them.
        match &last_end_pos {
            Some(last_end_pos) if begin_pos < *last_end_pos => continue,
            _ => {}
        }

        // Find the next end directive that follows this begin directive.
        let mut end_pos: Option<BytePos>;
        loop {
            end_pos = exclusion_end_iter.next();
            match end_pos {
                Some(end_pos) if end_pos <= begin_pos => continue,
                _ => break,
            }
        }

        // If this begin directive has a corresponding end directive, generate a span between
        // the two directives. Otherwise, generate a span ranging from the begin directive to
        // the end of the file.
        match end_pos {
            Some(end_pos) => {
                last_end_pos = Some(end_pos);
                privacy_allowlist_excluded_spans.push(Span {
                    lo: begin_pos,
                    hi: end_pos,
                });
            }
            None => {
                last_end_pos = Some(file.end_pos);
                privacy_allowlist_excluded_spans.push(Span {
                    lo: begin_pos,
                    hi: file.end_pos,
                });
            }
        }
    }

    privacy_allowlist_excluded_spans.sort_unstable();
}

fn bounds_of_lines_intersecting_span(span: Span, file: &InputFile) -> Option<Span> {
    let lo_span = bounds_of_line_containing_pos(span.lo, file);
    let hi_span = bounds_of_line_containing_pos(span.hi, file);
    match (lo_span, hi_span) {
        (Some(lo_span), Some(hi_span)) => Some(Span {
            lo: lo_span.lo,
            hi: hi_span.hi,
        }),
        (Some(lo_span), None) => Some(Span {
            lo: lo_span.lo,
            hi: lo_span.hi,
        }),
        (None, Some(hi_span)) => Some(Span {
            lo: hi_span.lo,
            hi: hi_span.hi,
        }),
        (None, None) => None,
    }
}

fn bounds_of_line_containing_pos(pos: BytePos, file: &InputFile) -> Option<Span> {
    let source_file_and_line = file.map.lookup_line(pos).ok()?;
    let line_bounds = source_file_and_line
        .sf
        .line_bounds(source_file_and_line.line);
    Some(Span {
        lo: line_bounds.0,
        hi: line_bounds.1,
    })
}

fn bounds_of_line_after_pos(pos: BytePos, file: &InputFile) -> Option<Span> {
    let source_file_and_line = file.map.lookup_line(pos).ok()?;
    let line_bounds = source_file_and_line
        .sf
        .line_bounds(source_file_and_line.line + 1);
    Some(Span {
        lo: line_bounds.0,
        hi: line_bounds.1,
    })
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
