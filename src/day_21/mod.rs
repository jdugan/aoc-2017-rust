pub mod helpers;

use std::collections::HashMap;

use crate::day_21::helpers::{ Rule, Tile };
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    21
}

pub fn puzzle1() -> u32 {
    let tile = perform_expansions(5);
    tile.count()
}

pub fn puzzle2() -> u32 {
    let tile = perform_expansions(18);
    tile.count()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn perform_expansions(loops: usize) -> Tile {
    let rules    = data();
    let seed     = ".#./..#/###".to_string();
    let mut tile = Tile::from(&seed);
    for _ in 0..loops {
        tile = tile.expand(&rules);
    }
    tile
}


// ========== DATA ========================================

fn data() -> HashMap<String, String> {
    let mut rules: HashMap<String, String> = HashMap::new();
    for line in reader::to_strings("./data/day21/input.txt") {
        let mut parts = line.split(" => ");
        let base_key  = parts.next().unwrap().to_string();
        let rule      = Rule{ key: base_key };
        let keys      = rule.permutated_keys();
        let val       = parts.next().unwrap();
        for k in keys {
            rules.insert(k, val.to_string());
        }
    }
    rules
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_21() {
        assert_eq!(puzzle1(), 173);
        assert_eq!(puzzle2(), 2456178);
    }
}