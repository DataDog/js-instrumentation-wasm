use std::{cmp::max, collections::HashMap};

use swc_ecma_ast::Ident;

pub struct IdentifierTracker {
    desired_identifier_availability: HashMap<String, bool>,
    tracker: CharacterTracker,
}

impl IdentifierTracker {
    pub fn new(desired_identifiers: Vec<&str>) -> IdentifierTracker {
        let mut tracker = IdentifierTracker {
            desired_identifier_availability: HashMap::new(),
            tracker: CharacterTracker::new(),
        };

        for desired_identifier in desired_identifiers {
            tracker
                .desired_identifier_availability
                .insert(desired_identifier.into(), true);
        }

        tracker
    }

    pub fn add_ident(self: &mut Self, identifier: &Ident) {
        let string: &str = identifier.sym.as_str();
        self.update_desired_identifier_availability(string);
        self.tracker.add(string);
    }

    pub fn new_unused_identifier(self: &mut Self, desired_identifier: &str) -> String {
        match self.desired_identifier_availability.get(desired_identifier) {
            Some(true) => {
                return desired_identifier.into();
            }
            _ => {}
        };

        // Generate a new, unused identifier. We do this by selecting a character which is known to
        // be unused in any identifier at a certain string position, and then generating an
        // identifier which places the character at this position.
        let (unused_char, position) = self.tracker.first_unused_character();
        let unused_identifier = format!("{}{}", "D".repeat(position), unused_char);

        // Record the fact that we used this identifier, so we won't try to reuse it if this
        // function is called again.
        self.tracker.add(&unused_identifier);

        unused_identifier
    }

    fn update_desired_identifier_availability(self: &mut Self, string: &str) {
        match self.desired_identifier_availability.get(string) {
            Some(true) => {
                self.desired_identifier_availability
                    .insert(string.into(), false);
            }
            _ => {}
        }
    }
}

struct CharacterTracker {
    // An array that maps valid JS identifier characters to the first position at which we've never
    // seen them occur. There are 64 characters to consider:
    // * 26 uppercase letters
    // * 26 lowercase letters
    // * 10 digits
    // * '$'
    // * '_'
    pub characters: [usize; 64],
}

impl CharacterTracker {
    pub fn new() -> CharacterTracker {
        let mut set = CharacterTracker {
            characters: [0; 64],
        };

        // A JS identifier can't begin with a digit. To ensure that we don't generate invalid
        // identifiers, we pretend that we've seen every digit at index 0.
        for digit in '0'..='9' {
            set.add(&digit.to_string());
        }

        // Some internationalization libraries require you to wrap string literals in a function
        // called "__()". To minimize the risk of any confusion, ensure that we never generate an
        // identifier that would collide with this function.
        set.add("__");

        set
    }

    pub fn add(self: &mut Self, string: &str) {
        for (position, character) in string.chars().enumerate() {
            match Self::index_for(character) {
                Some(index) => {
                    self.characters[index] = max(self.characters[index], position + 1);
                }
                None => {}
            }
        }
    }

    pub fn first_unused_character(self: &Self) -> (char, usize) {
        self.characters
            .iter()
            .enumerate()
            .min_by(|(_, a_position), (_, b_position)| a_position.cmp(b_position))
            .map(|(index, position)| (Self::character_for(index).unwrap(), position.clone()))
            .unwrap()
    }

    fn character_for(index: usize) -> Option<char> {
        match index {
            // Uppercase letters: indices 0 to 25.
            0..=25 => Some(('A' as u8 + index as u8) as char),
            // Lowercase letters: indices 26 to 51.
            26..=51 => Some(('a' as u8 + index as u8) as char),
            // Digits: indices 52 to 61.
            52..=61 => Some(('0' as u8 + index as u8) as char),
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
