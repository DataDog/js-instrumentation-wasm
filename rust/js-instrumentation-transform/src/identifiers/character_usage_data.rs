use std::cmp::max;

/// Metadata about where a character has been used in identifiers
/// we've encountered in the input so far.
#[derive(Debug)]
pub struct CharacterUsageData {
    /// This bit mask tracks whether the character has appeared in
    /// the first 16 positions within any identifier. If a bit is
    /// set to 1, it means that the character has appeared in that
    /// position. For example, for the character 'x', if we've seen
    /// the identifiers 'x' and 'box', 'low_pos_mask' will be:
    ///   0b0000000000000101
    pub low_pos_mask: u16,

    /// This number tracks the lowest position greater than every
    /// position at which we've seen the character. For example,
    /// for the 'x' and 'box' example above, 'upper_bound' will be
    /// 3, because the highest position we've seen for the character
    /// 'x' is at index 2 in 'box'.
    pub upper_bound: u16,
}

impl CharacterUsageData {
    pub const DEFAULT: CharacterUsageData = CharacterUsageData {
        low_pos_mask: 0,
        upper_bound: 0,
    };

    pub fn add_usage(self: &mut Self, position: usize) {
        let pos_16: u16 = position.try_into().unwrap_or(u16::MAX);
        if pos_16 < 16 {
            self.low_pos_mask = self.low_pos_mask | (1 << pos_16);
        }

        if pos_16 == u16::MAX {
            self.upper_bound = u16::MAX
        } else {
            self.upper_bound = max(self.upper_bound, pos_16 + 1);
        }
    }

    pub fn first_unused(self: &Self) -> usize {
        if self.low_pos_mask < u16::MAX {
            for position in 0..16 {
                if self.low_pos_mask & (1 << position) == 0 {
                    return position;
                }
            }
        }

        self.upper_bound as usize
    }
}

impl Default for CharacterUsageData {
    fn default() -> Self {
        Self::DEFAULT
    }
}
