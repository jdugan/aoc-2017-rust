use std::collections::HashSet;

use crate::utility::reader;


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

// ========== SOLUTIONS ===================================

fn anagram_free_count() -> u32 {
    let mut sum: u32 = 0;
    for p0 in data() {
        let p1: Vec<String> = p0.into_iter().map(|w| sort_letters(w)).collect();
        let p2: HashSet<String> = HashSet::from_iter(p1.clone());
        if p1.len() == p2.len() {
            sum += 1;
        }
    }
    sum
}

fn duplicate_free_count() -> u32 {
    let mut sum: u32 = 0;
    for p0 in data() {
        let p1: HashSet<String> = HashSet::from_iter(p0.clone());
        if p0.len() == p1.len() {
            sum += 1;
        }
    }
    sum
}

// ========== HELPERS =====================================

fn sort_letters(word: String) -> String {
    let mut v = reader::split_word(word);
    v.sort_unstable();
    v.join("")
}

// ========== DATA ========================================

fn data() -> Vec<Vec<String>> {
    reader::to_lines("./data/day04/input.txt")
        .into_iter()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Vec<String> {
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