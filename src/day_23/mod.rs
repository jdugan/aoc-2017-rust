pub mod helpers;

use crate::day_23::helpers::{ Command, Computer };
use crate::utility::{ converter, reader };


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    23
}

pub fn puzzle1() -> u32 {
    let     program = data();
    let mut computer = Computer::new(program);
    computer.run()
}

pub fn puzzle2() -> u32 {
    let program = data();
    let computer = Computer::new(program);
    computer.shortcut()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== DATA ========================================

fn data() -> Vec<Command> {
    reader::to_strings("./data/day23/input.txt")
        .iter()
        .map(|s| {
            let mut args = converter::string_to_words(s);
            args.reverse();
            let action   = args.pop().unwrap();
            let register = args.pop().unwrap();
            Command{ action, register, args }
        })
        .collect()
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_23() {
        assert_eq!(puzzle1(), 6724);
        assert_eq!(puzzle2(), 903);
    }
}