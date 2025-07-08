use std::collections::HashMap;

use swc_ecma_ast::{Ident, IdentName};

use super::character_tracker::CharacterTracker;

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

    pub fn add_ident_name(self: &mut Self, identifier: &IdentName) {
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

        // Generate a new, unused identifier. We do this by selecting a character which is
        // known not to appear at the end of any identifier of a certain length, and then
        // generating an identifier with that length which ends in the selected character.
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
