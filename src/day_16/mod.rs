pub mod helpers;

use crate::day_16::helpers::Party;
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    16
}

pub fn puzzle1() -> String {
    let mut party = Party::new(16_u8, data());
    party.dance()
}

pub fn puzzle2() -> String {
    let mut party = Party::new(16_u8, data());
    party.dance_all_night()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== DATA ========================================

fn data() -> Vec<String> {
    reader::to_word("./data/day16/input.txt")
        .split(",")
        .map(String::from)
        .collect()
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_16() {
        assert_eq!(puzzle1(), "jcobhadfnmpkglie".to_string());
        assert_eq!(puzzle2(), "pclhmengojfdkaib".to_string());
    }
}