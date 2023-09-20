pub mod helpers;

use crate::day_18::helpers::{ Command, Computer };
use crate::utility::{ converter, reader };


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    18
}

pub fn puzzle1() -> i64 {
    let     program  = data();
    let mut computer = Computer::new(0, program);
    let     _        = computer.run();
    computer.output.pop().unwrap()
}

pub fn puzzle2() -> u64 {
    let program   = data();
    let mut count = 0_u64;
    let mut c0    = Computer::new(0, program.clone());
    let mut c1    = Computer::new(1, program.clone());
    loop {
        // run both computers
        let _    = c0.run();
        let _    = c1.run();
        count   += c1.output.len() as u64;

        // move outputs
        c0.input = c1.output.clone();
        c1.input = c0.output.clone();
        c0.output = vec![];
        c1.output = vec![];

        // halt check
        if c0.is_deadlocked() && c1.is_deadlocked() {
            break;
        }
    }
    count
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== DATA ========================================

fn data() -> Vec<Command> {
    reader::to_strings("./data/day18/input.txt")
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
    fn test_day_18() {
        assert_eq!(puzzle1(), 1187);
        assert_eq!(puzzle2(), 5969);
    }
}