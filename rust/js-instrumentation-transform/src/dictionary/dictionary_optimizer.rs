use std::cmp::Ordering;

use super::{dictionary_error::DictionaryError, Dictionary};

pub struct OptimizedDictionary {
    pub indices: Vec<usize>,
    pub strings: Dictionary,
}

impl OptimizedDictionary {
    pub fn build(dictionary_identifier: &String, mut strings: Dictionary) -> OptimizedDictionary {
        let index = Self::optimize(dictionary_identifier, &mut strings);
        OptimizedDictionary {
            indices: index,
            strings,
        }
    }

    pub fn entry_for_index(self: &Self, dictionary_index: usize) -> Result<usize, DictionaryError> {
        match self.strings.get_index(dictionary_index) {
            Some((_, stats)) => Ok(stats.dictionary_entry),
            None => Err(DictionaryError::InvalidIndex(dictionary_index)),
        }
    }

    fn optimize(dictionary_identifier: &String, strings: &mut Dictionary) -> Vec<usize> {
        let mut index: Vec<usize> = (0..strings.len()).into_iter().collect();
        index.sort_by(|a, b| {
            match (strings.get_index(*a), strings.get_index(*b)) {
                (Some((ref a_key, ref a_val)), Some((ref b_key, ref b_val))) => {
                    // Earlier entries in the dictionary have lower, and thus shorter, indices,
                    // which means that dictionary references to those entries will be shorter.
                    // (e.g., D[1] is shorter than D[1000]). We never replace a string with a
                    // dictionary reference unless the reference is shorter than the original
                    // string, so the order of the dictionary entries affects which strings we'll
                    // replace with dictionary references, and how many characters those dictionary
                    // references require, and this in turn affects the overall size of the output.
                    // This means we have an optimization problem to solve: for example, it's
                    // better if strings that appear many times get shorter indices. The current
                    // solution uses an ordering derived from the maximum possible benefit we could
                    // see from substituting each dictionary entry, if it received the shortest
                    // possible index. This is not optimal, but it should be better than a naive
                    // ordering.
                    let ordering = b_key
                        .max_dict_ref_benefit(&dictionary_identifier, b_val.count)
                        .cmp(&a_key.max_dict_ref_benefit(&dictionary_identifier, a_val.count));

                    // Within each equivalence class of strings, order strings that appear earlier
                    // before strings that appear later. We do this to ensure that the output is
                    // stable.
                    match ordering {
                        Ordering::Equal => a_val.index.cmp(&b_val.index),
                        other => other,
                    }
                }
                // If one value is None (which is a bug, but we should still handle it),
                // sort it at the end of the list.
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                (None, None) => Ordering::Equal,
            }
        });

        for (dictionary_entry, index) in index.iter().enumerate() {
            match strings.get_index_mut(*index) {
                Some((_, val)) => val.dictionary_entry = dictionary_entry,
                None => {}
            }
        }

        index
    }
}
