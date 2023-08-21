use std::collections::HashSet;

use crate::utility::{ converter, reader };


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    4
}

pub fn puzzle1() -> u32 {
    duplicate_free_count()
}

pub fn puzzle2() -> u32 {
    anagram_free_count()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn anagram_free_count() -> u32 {
    data().iter().fold(0_u32, |sum, p0| {
        let p1: Vec<String>     = p0.into_iter().map(|w| sort_letters(w)).collect();
        let p2: HashSet<String> = HashSet::from_iter(p1.clone());
        match p1.len() == p2.len() {
            true  => sum + 1,
            false => sum
        }
    })
}

fn duplicate_free_count() -> u32 {
    data().iter().fold(0_u32, |sum, p0| {
        let p1: HashSet<String> = HashSet::from_iter(p0.clone());
        match p0.len() == p1.len() {
            true  => sum + 1,
            false => sum
        }
    })
}


// ========== HELPERS =====================================

fn sort_letters(word: &String) -> String {
    let mut v = converter::string_to_letters(word);
    v.sort_unstable();
    v.join("")
}


// ========== DATA ========================================

fn data() -> Vec<Vec<String>> {
    reader::to_strings("./data/day04/input.txt")
        .iter()
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &String) -> Vec<String> {
    line.split_whitespace()
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
    fn test_day_04() {
        assert_eq!(puzzle1(), 455);
        assert_eq!(puzzle2(), 186);
    }
}