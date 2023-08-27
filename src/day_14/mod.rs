use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::crypto::KnotHash;
use crate::common::grid::Point;
use crate::utility::{ converter, reader };


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    14
}

pub fn puzzle1() -> u32 {
    grid().into_values().sum()
}

pub fn puzzle2() -> usize {
    let points = grid();
    let groups = build_groups(&points);
    groups.len()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== HELPERS =====================================

fn build_groups(points: &HashMap<(i32, i32), u32>) -> Vec<Vec<(i32, i32)>> {
    let mut groups: Vec<Vec<(i32, i32)>> = vec![];
    for (id, _) in points {
        groups.push(generate_group(points, id));
    }
    groups.sort_unstable();
    groups.dedup();
    groups
}

fn generate_group(points: &HashMap<(i32, i32), u32>, id: &(i32, i32)) -> Vec<(i32, i32)> {
    //  find unique keys
    let mut check_keys: HashSet<(i32, i32)> = HashSet::new();
    let mut found_keys: HashSet<(i32, i32)> = HashSet::new();
    check_keys.insert(id.clone());
    found_keys.insert(id.clone());
    while check_keys.len() > 0 {
        let loop_keys = check_keys.clone();
        check_keys = HashSet::new();
        for key in loop_keys {
            let (x, y) = key;
            let cids = Point::new(x, y).cardinal_ids();
            for cid in cids {
                if Some(&1) == points.get(&cid) {
                    if found_keys.insert(cid.clone()) {
                        check_keys.insert(cid.clone());
                    }
                }
            }
        }
    }
    // organise into a sorted vector
    let mut group = Vec::from_iter(found_keys);
    group.sort_unstable();
    group
}

// ========== DATA ========================================

fn data() -> String {
    reader::to_word("./data/day14/input.txt")
}

fn grid() -> HashMap<(i32, i32), u32> {
    let mut grid = HashMap::new();
    for (ux, hash) in knot_hashes().iter().enumerate() {
        let bs = hash.to_binary();
        for (uy, v) in converter::string_to_letters(&bs)
                        .iter()
                        .map(|s| s.parse::<u32>().unwrap())
                        .enumerate() {
            if v > 0 {
                let x = ux as i32;
                let y = uy as i32;
                grid.insert((x, y), v);
            }
        }
    }
    grid
}

fn knot_hashes() -> Vec<KnotHash> {
    let root = data();
    (0..128).fold(vec![], |mut arr, n| {
        let     seed = format!("{0}-{1}", root, n);
        let mut hash = KnotHash::new(seed);
        hash.shuffle();
        arr.push(hash);
        arr
    })
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_14() {
        assert_eq!(puzzle1(), 8190);
        assert_eq!(puzzle2(), 2);
    }
}