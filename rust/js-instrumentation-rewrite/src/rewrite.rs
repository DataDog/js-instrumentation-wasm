use std::{cmp::Ordering, fmt::Display};

use swc_common::{BytePos, Span};

use crate::rewrite_content::RewriteContent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Rewrite<Content: RewriteContent> {
    Insert { content: Content, pos: BytePos },
    Replace { content: Content, span: Span },
}

impl<Content: RewriteContent> Rewrite<Content> {
    pub fn content(self: &Self) -> &Content {
        match self {
            Rewrite::Insert { content, .. } => content,
            Rewrite::Replace { content, .. } => content,
        }
    }

    pub fn lo(self: &Self) -> &BytePos {
        match self {
            Rewrite::Insert { pos, .. } => pos,
            Rewrite::Replace { span, .. } => &span.lo,
        }
    }

    pub fn hi(self: &Self) -> &BytePos {
        match self {
            Rewrite::Insert { pos, .. } => pos,
            Rewrite::Replace { span, .. } => &span.hi,
        }
    }

    pub fn span(self: &Self) -> Span {
        match self {
            Rewrite::Insert { pos, .. } => (*pos, *pos).into(),
            Rewrite::Replace { span, .. } => *span,
        }
    }

    pub fn filter_map_content<NewContent: RewriteContent, F>(
        self: &Self,
        transform: F,
    ) -> Option<Rewrite<NewContent>>
    where
        F: FnOnce(&Content) -> Option<NewContent>,
    {
        match self {
            Rewrite::Insert { content, pos } => Some(Rewrite::Insert {
                content: transform(content)?,
                pos: *pos,
            }),
            Rewrite::Replace { content, span } => Some(Rewrite::Replace {
                content: transform(content)?,
                span: *span,
            }),
        }
    }

    pub fn map_content<NewContent: RewriteContent, F>(
        self: &Self,
        transform: F,
    ) -> Rewrite<NewContent>
    where
        F: FnOnce(&Content) -> NewContent,
    {
        match self {
            Rewrite::Insert { content, pos } => Rewrite::Insert {
                content: transform(content),
                pos: *pos,
            },
            Rewrite::Replace { content, span } => Rewrite::Replace {
                content: transform(content),
                span: *span,
            },
        }
    }
}

impl<Content: RewriteContent> Display for Rewrite<Content> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rewrite::Insert { content, pos } => {
                write!(f, "[{}] Insert: {}", pos.0, content)
            }
            Rewrite::Replace { content, span } => {
                write!(f, "[{}, {}) Replace: {}", span.lo.0, span.hi.0, content)
            }
        }
    }
}

impl<Content: RewriteContent> PartialOrd for Rewrite<Content> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Content: RewriteContent> Ord for Rewrite<Content> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Place rewrites with earlier starting positions before rewrites with later starting
        // positions.
        match self.lo().cmp(&other.lo()) {
            Ordering::Equal => {}
            other => return other,
        };

        // When an insert and a replace appear at the same starting position, place the
        // insert before the replace.
        match (self, other) {
            (Rewrite::Insert { .. }, Rewrite::Replace { .. }) => return Ordering::Less,
            (Rewrite::Replace { .. }, Rewrite::Insert { .. }) => return Ordering::Greater,
            (Rewrite::Insert { .. }, Rewrite::Insert { .. }) => {}
            (Rewrite::Replace { .. }, Rewrite::Replace { .. }) => {}
        }

        // When rewrites have the same starting position, place rewrites with later ending
        // positions before rewrites with earlier starting positions. When we filter out
        // conflicting rewrites that overlap, we keep rewrites that are earlier in the sort
        // order, so this has the effect that we keep the rewrite that affects more of the original
        // source code when two rewrites target the same position.
        match self.hi().cmp(&other.hi()) {
            Ordering::Equal => {}
            Ordering::Greater => return Ordering::Less,
            Ordering::Less => return Ordering::Greater,
        };

        // Finally, order content according to its Ord implementation. Ord can be used to express
        // simple dependencies between different kinds of content. (The main use case for this is
        // to allow us to insert multiple items at the beginning or end of the input while
        // maintaining control over the order in which they'll appear in the output.)
        self.content().cmp(&other.content())
    }
}
