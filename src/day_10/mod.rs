use crate::common::crypto::KnotHash;
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    10
}

pub fn puzzle1() -> u32 {
    let mut hash = KnotHash::new(data());
    hash.quick_shuffle();
    hash.checksum()
}

pub fn puzzle2() -> String {
    let mut hash = KnotHash::new(data());
    hash.shuffle();
    hash.to_hex()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== DATA ========================================

fn data() -> String {
    reader::to_word("./data/day10/input.txt")
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10() {
        assert_eq!(puzzle1(), 212);
        assert_eq!(puzzle2(), "96de9657665675b51cd03f0b3528ba26".to_string());
    }
}