use std::fmt::Display;

pub trait RewriteKind: Clone + Display + Eq + Ord {}
impl<T: Clone + Display + Eq + Ord> RewriteKind for T {}
