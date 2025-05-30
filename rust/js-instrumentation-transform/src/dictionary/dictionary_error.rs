use std::fmt::Display;

pub enum DictionaryError {
    InvalidIndex(usize),
}

impl Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DictionaryError::InvalidIndex(index) => {
                write!(f, "Invalid dictionary index {}", index)
            }
        }
    }
}
