use std::collections::HashMap;

use crate::utility::reader;


// ----------------------------------------------------
// Public Methods
// ----------------------------------------------------

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


// ----------------------------------------------------
// Private Methods
// ----------------------------------------------------

// ========== SOLUTIONS ===============================

fn block_to_drain(blocks: &Vec<i32>) -> (usize, i32) {
    let mut best_i: usize = 0;
    let mut best_v: i32   = -1;
    for (i, v) in blocks.iter().enumerate() {
        if v > &best_v {
            best_i = i;
            best_v = *v;
        }
    }
    (best_i, best_v)
}

fn cycle_information() -> (u32, u32) {
    let mut history: HashMap<Vec<i32>, u32> = HashMap::new();
    let mut blocks: Vec<i32> = data();
    let mut cycles: u32      = 0;
    let mut repeat: u32      = 0;
    let     size:   usize    = blocks.len();
    history.insert(blocks.clone(), cycles);

    loop {
        cycles += 1;
        let (bi, bv) = block_to_drain(&blocks);
        blocks[bi]   = 0;
        for n in 0..(bv as usize) {
            let i = (bi + 1 + n) % size;
            blocks[i] += 1;
        }
        let key = blocks.clone();
        match history.get(&key) {
            Some(&marker) => {
                repeat = cycles - marker;
                break;
            },
            None => {
                history.insert(key, cycles);
            }
        }
    }
    (cycles, repeat)
}


// ========== DATA ====================================

fn data() -> Vec<i32> {
    reader::to_lines("./data/day06/input.txt")
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}


// ----------------------------------------------------
// Unit Tests
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06() {
        assert_eq!(puzzle1(), 4074);
        assert_eq!(puzzle2(), 2793);
    }
}