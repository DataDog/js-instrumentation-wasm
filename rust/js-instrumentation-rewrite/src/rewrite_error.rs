use std::{error::Error, fmt};

#[derive(Debug)]
pub struct RewriteError {
    pub reason: String,
}

impl RewriteError {
    pub fn new(reason: String) -> Self {
        RewriteError { reason }
    }
}

impl fmt::Display for RewriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error rewriting code: {}", self.reason)
    }
}

impl Error for RewriteError {}
