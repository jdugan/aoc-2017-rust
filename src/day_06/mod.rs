use std::collections::HashMap;

use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    6
}

pub fn puzzle1() -> u32 {
    let (total_count, _) = cycle_information();
    total_count
}

pub fn puzzle2() -> u32 {
    let (_, loop_count) = cycle_information();
    loop_count
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =================================

fn cycle_information() -> (u32, u32) {
    let mut history: HashMap<Vec<u32>, u32> = HashMap::new();
    let mut banks: Vec<u32> = data();
    history.insert(banks.clone(), 0_32);

    let mut cycles = 0_u32;
    let mut repeat = 0_u32;
    let     size   = banks.len();

    while repeat == 0 {
        let (pos, blocks) = bank_to_drain(&banks);
        cycles     += 1;
        banks[pos]  = 0;
        for n in 0..(blocks as usize) {
            let i = (pos + 1 + n) % size;
            banks[i] += 1;
        }

        let key = banks.clone();
        match history.get(&key) {
            Some(&marker) => {
                repeat = cycles - marker;
            },
            None => {
                history.insert(key, cycles);
            }
        }
    }
    (cycles, repeat)
}


// ========== HELPERS =================================

fn bank_to_drain(banks: &Vec<u32>) -> (usize, u32) {
    banks.iter().enumerate().fold((0, 0_u32), |best, (i, v)| {
        match *v > best.1 {
            true  => (i, *v),
            false => best,
        }
    })
}


// ========== DATA ====================================

fn data() -> Vec<u32> {
    reader::to_unsigned_ints("./data/day06/input.txt")
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06() {
        assert_eq!(puzzle1(), 4074);
        assert_eq!(puzzle2(), 2793);
    }
}