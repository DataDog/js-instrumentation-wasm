use std::fmt::Display;

use swc_common::BytePos;

pub trait RewriteContent: Clone + Display + Eq + Ord {
    /// This function should return a source BytePos if this rewrite needs a source position
    /// mapping that would be different from the default. Most rewrites don't need this, because
    /// their source mapping just points to the position in the source that corresponds to their
    /// position in the output stream; this kind of mapping is added by default. However, if you
    /// want to map a rewrite to a totally different place in the source, then you'll need to
    /// return that place from this function.
    fn source_pos(self: &Self) -> Option<BytePos>;
}
