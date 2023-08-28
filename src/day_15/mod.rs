pub mod helpers;

use crate::day_15::helpers::Generator;
use crate::utility::{ converter, reader };


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    15
}

pub fn puzzle1() -> u32 {
    let divisor1: u64 = 1;
    let divisor2: u64 = 1;
    let rounds:   u64 = 40000000;
    count_matches(divisor1, divisor2, rounds)
}

pub fn puzzle2() -> u32 {
    let divisor1: u64 = 4;
    let divisor2: u64 = 8;
    let rounds:   u64 = 5000000;
    count_matches(divisor1, divisor2, rounds)
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== HELPERS =====================================

fn compare(v1: &u64, v2: &u64) -> bool {
    let d0: u64 = 2_u64.pow(16);
    let c1: u64 = v1 % d0;
    let c2: u64 = v2 % d0;
    c1 == c2
}

fn count_matches(divisor1: u64, divisor2: u64, rounds: u64) -> u32 {
    let (mut g1, mut g2) = generators(divisor1, divisor2);
    let mut count        = 0_u32;
    for _ in 0..rounds {
        g1.next();
        g2.next();
        if compare(&g1.value, &g2.value) {
            count += 1;
        }
    }
    count
}


// ========== DATA ========================================

fn data() -> Vec<u64> {
    reader::to_strings("./data/day15/input.txt")
        .iter()
        .map(|line| {
            let v = converter::string_to_words(line);
            let s = &v[v.len() - 1];
            s.parse::<u64>().unwrap()
        })
        .collect()
}

fn generators(divisor1: u64, divisor2: u64) -> (Generator, Generator) {
    let seeds = data();
    let g1    = Generator::new(seeds[0], 16807_u64, divisor1);
    let g2    = Generator::new(seeds[1], 48271_u64, divisor2);
    (g1, g2)
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_15() {
        assert_eq!(puzzle1(), 569);
        assert_eq!(puzzle2(), 298);
    }
}