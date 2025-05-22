mod dictionary_optimizer;
pub use dictionary_optimizer::OptimizedDictionary;

mod dictionary_tracker;
pub use dictionary_tracker::{Dictionary, DictionaryEntry, DictionaryTracker};

pub const DEFAULT_DICTIONARY_IDENTIFIER: &'static str = "D";
