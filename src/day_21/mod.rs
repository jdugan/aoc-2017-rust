pub mod helpers;

use std::collections::HashMap;

use crate::day_21::helpers::{ Grid, Image };
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    21
}

pub fn puzzle1() -> u32 {
    println!("{:?}", data());
    1
}

pub fn puzzle2() -> u32 {
    2
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== DATA ========================================

fn data() -> HashMap<String, String> {
    let mut rules: HashMap<String, String> = HashMap::new();
    for line in reader::to_strings("./data/day21/input.txt") {
        let mut parts = line.split(" => ");
        let base_key  = parts.next().unwrap().to_string();
        let image     = Image{ key: base_key };
        let keys      = image.permutated_keys();
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
        assert_eq!(puzzle1(), 1);
        assert_eq!(puzzle2(), 2);
    }
}