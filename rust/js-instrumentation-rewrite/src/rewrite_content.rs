use std::fmt::Display;

pub trait RewriteContent: Clone + Display + Eq + Ord {}
impl<T: Clone + Display + Eq + Ord> RewriteContent for T {}
