pub mod helpers;

use crate::day_09::helpers::Parser;
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    9
}

pub fn puzzle1() -> u32 {
    let mut parser = Parser{ output: data(), ..Default::default() };
    parser.remove_annotations();
    parser.remove_garbage();
    parser.remove_punctuation();
    parser.calculate_score();
    parser.score
}

pub fn puzzle2() -> u32 {
    let mut parser = Parser{ output: data(), ..Default::default() };
    parser.remove_annotations();
    parser.remove_garbage();
    parser.bin_size
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== DATA ========================================

fn data() -> String {
    reader::to_lines("./data/day09/input.txt")
        .pop()
        .unwrap()
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_09() {
        assert_eq!(puzzle1(), 16689);
        assert_eq!(puzzle2(), 7982);
    }
}