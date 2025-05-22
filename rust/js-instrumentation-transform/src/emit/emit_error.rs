use std::fmt::Display;

use crate::rewrite::ReplaceTarget;

pub enum EmitError {
    InvalidStringIndex(ReplaceTarget, usize),
}

impl Display for EmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmitError::InvalidStringIndex(target, index) => {
                write!(f, "invalid string index {} for {:?}", index, target)
            }
        }
    }
}
