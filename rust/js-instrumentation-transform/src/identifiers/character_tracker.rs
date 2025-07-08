use super::character_usage_data::CharacterUsageData;

pub struct CharacterTracker {
    // An array that maps valid JS identifier characters to usage data for
    // each character. There are 64 characters to consider:
    // * 26 uppercase letters
    // * 26 lowercase letters
    // * 10 digits
    // * '$'
    // * '_'
    pub characters: [CharacterUsageData; 64],
}

impl CharacterTracker {
    pub fn new() -> CharacterTracker {
        let mut set = CharacterTracker {
            characters: [CharacterUsageData::DEFAULT; 64],
        };

        // A JS identifier can't begin with a digit. To ensure that we don't generate invalid
        // identifiers, we pretend that we've seen every digit at index 0.
        for digit in '0'..='9' {
            set.add(&digit.to_string());
        }

        // Some internationalization libraries require you to wrap string literals in a function
        // called "_()" or "__()". To minimize the risk of any confusion, ensure that we never
        // generate an identifier that would collide with this function.
        set.add("_");
        set.add("__");

        set
    }

    pub fn add(self: &mut Self, string: &str) {
        match string.chars().enumerate().last() {
            Some((position, character)) => match Self::index_for(character) {
                Some(index) => {
                    self.characters[index].add_usage(position);
                }
                None => {}
            },
            None => {}
        }
    }

    pub fn first_unused_character(self: &Self) -> (char, usize) {
        self.characters
            .iter()
            .enumerate()
            .min_by(|(_, a_position), (_, b_position)| {
                a_position.first_unused().cmp(&b_position.first_unused())
            })
            .map(|(index, position)| (Self::character_for(index).unwrap(), position.first_unused()))
            .unwrap()
    }

    fn character_for(index: usize) -> Option<char> {
        match index {
            // Uppercase letters: indices 0 to 25.
            0..=25 => Some(('A' as u8 + index as u8) as char),
            // Lowercase letters: indices 26 to 51.
            26..=51 => Some(('a' as u8 + index as u8 - 26) as char),
            // Digits: indices 52 to 61.
            52..=61 => Some(('0' as u8 + index as u8 - 52) as char),
            // '$': index 62.
            62 => Some('$'),
            // '_': index 63.
            63 => Some('_'),
            // Ignore other characters.
            _ => None,
        }
    }

    fn index_for(character: char) -> Option<usize> {
        match character {
            // Uppercase letters: indices 0 to 25.
            'A'..='Z' => Some((character as u32 - 'A' as u32) as usize),
            // Lowercase letters: indices 26 to 51.
            'a'..='z' => Some((character as u32 - 'a' as u32) as usize + 26),
            // Digits: indices 52 to 61.
            '0'..='9' => Some((character as u32 - '0' as u32) as usize + 52),
            // '$': index 62.
            '$' => Some(62),
            // '_': index 63.
            '_' => Some(63),
            // Ignore other characters.
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::once;

    use super::*;

    #[test]
    fn uses_consistent_indices_for_characters() {
        for nondigit in ('A'..='Z').chain('a'..='z').chain(once('$')) {
            assert_character_has_consistent_index(nondigit, 0);
        }
        for digit in '0'..='9' {
            assert_character_has_consistent_index(digit, 1);
        }
        assert_character_has_consistent_index('_', 2);
    }

    fn assert_character_has_consistent_index(character: char, expected_first_unused_pos: usize) {
        // Check that an index exists for this character.
        let index = CharacterTracker::index_for(character);
        assert!(index.is_some());
        let index = index.unwrap();

        // Check that mapping we can map the index back to the original character.
        assert_eq!(CharacterTracker::character_for(index), Some(character));

        // Check that the first unused position for this character is what we expect.
        let mut character_tracker = CharacterTracker::new();
        assert_eq!(
            character_tracker.characters[index].first_unused(),
            expected_first_unused_pos
        );

        // Check that adding an instance of this character changes the first unused position.
        character_tracker.add(&character.to_string().repeat(expected_first_unused_pos + 1));
        assert_eq!(
            character_tracker.characters[index].first_unused(),
            expected_first_unused_pos + 1
        );
    }

    #[test]
    fn finds_an_unused_character() {
        let mut character_tracker = CharacterTracker::new();

        // Add all single character identifiers except 'z'.
        for_every_identifier_character(|character| {
            if character == 'z' {
                return;
            }
            character_tracker.add(&character.to_string());
        });

        assert_eq!(character_tracker.first_unused_character(), ('z', 0));
    }

    #[test]
    fn finds_an_unused_character_when_all_single_character_identifiers_are_used() {
        let mut character_tracker = CharacterTracker::new();

        // Add all single character identifiers.
        for_every_identifier_character(|character| {
            character_tracker.add(&character.to_string());
        });

        assert_eq!(character_tracker.first_unused_character(), ('A', 1));
    }

    #[test]
    fn finds_an_unused_character_when_all_characters_at_pos_1_are_used() {
        let mut character_tracker = CharacterTracker::new();

        // Add an example of every character at position 1: e.g. "Aa", "Ab",
        // "Ac", etc. We should still be able to find an available character
        // at position 0.
        for_every_identifier_character(|character| {
            character_tracker.add(&format!("A{}", character.to_string()));
        });

        assert_eq!(character_tracker.first_unused_character(), ('A', 0));
    }

    #[test]
    fn finds_an_unused_character_when_all_characters_at_pos_0_and_1_are_used() {
        let mut character_tracker = CharacterTracker::new();

        // Add an example of every character at position 0: e.g. "A", "B",
        // "C", etc.
        for_every_identifier_character(|character| {
            character_tracker.add(&character.to_string());
        });

        // Add an example of every character at position 1: e.g. "Aa", "Ab",
        // "Ac", etc.
        for_every_identifier_character(|character| {
            character_tracker.add(&format!("A{}", character.to_string()));
        });

        // We should consider the first unused character to be at position 2.
        assert_eq!(character_tracker.first_unused_character(), ('A', 2));
    }

    fn for_every_identifier_character<F>(mut f: F)
    where
        F: FnMut(char) -> (),
    {
        for character in ('A'..='Z')
            .chain('a'..='z')
            .chain('0'..='9')
            .chain(once('$'))
            .chain(once('_'))
        {
            f(character);
        }
    }
}
