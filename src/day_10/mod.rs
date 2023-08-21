pub mod helpers;

use crate::day_10::helpers::List;
use crate::utility::{ converter, reader };


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    10
}

pub fn puzzle1() -> u32 {
    let init    = List::new(256_u32);
    let lengths = data_as_ints();
    let rounds  = 1_u32;
    let list    = generate_sparse_hash(init, lengths, rounds);
    list.checksum()
}

pub fn puzzle2() -> String {
    let init    = List::new(256_u32);
    let lengths = data_as_asciis(vec![17_u32, 31, 73, 47, 23]);
    let rounds  = 64_u32;
    let list    = generate_sparse_hash(init, lengths, rounds);
    let hex     = generate_knot_hash(&list);
    hex
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLUTIONS ===================================

fn generate_knot_hash(list: &List) -> String {
    let     values = list.sorted_values();
    let     chunks = values.chunks(16);
    let mut hash   = "".to_string();
    for c in chunks {
        let i = c.iter().fold(0, |a,b| a ^ b);
        hash  = format!("{}{:0>2x}", hash, i);
    }
    hash
}

fn generate_sparse_hash(mut list: List, lengths: Vec<u32>, rounds: u32) -> List {
    let mut skip = 0_u32;
    let mut pos  = 0_u32;
    for _ in 0..rounds {
        for length in &lengths {
            list.hash(*length, pos);
            pos   = (pos + length + skip) % &list.length;
            skip += 1;
        }
    }
    list
}

// ========== DATA ========================================

fn data() -> String {
    reader::to_word("./data/day10/input.txt")
}

fn data_as_asciis(suffix: Vec<u32>) -> Vec<u32> {
    let     s = data();
    let mut v = converter::string_to_asciis(&s);
    v.extend(suffix);
    v
}

fn data_as_ints() -> Vec<u32> {
    let s = data();
    converter::string_to_ints(&s)
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_00() {
        assert_eq!(puzzle1(), 212);
        assert_eq!(puzzle2(), "96de9657665675b51cd03f0b3528ba26".to_string());
    }
}