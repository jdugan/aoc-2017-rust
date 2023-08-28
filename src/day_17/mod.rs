use std::collections::HashMap;

use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    17
}

pub fn puzzle1() -> u32 {
    let target: u32 = 2017;
    let skip: u32   = data();
    arbitrary_neighbor_at(target, skip)
}

pub fn puzzle2() -> u32 {
    let target: u32 = 50000000;
    let skip: u32   = data();
    origin_neighbor_at(target, skip)
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn arbitrary_neighbor_at(target: u32, skip: u32) -> u32 {
    let mut spinlock: HashMap<u32, u32> = HashMap::from([(0, 1), (1, 0)]);
    let mut curr_id: u32 = 1;
    for id in 2..target+1 {
        let steps = skip % id;
         for _ in 0..steps {
            curr_id = spinlock.get(&curr_id).unwrap().clone();
        }
        let next_id = spinlock.get(&curr_id).unwrap().clone();
        spinlock.insert(curr_id, id.clone());
        spinlock.insert(id, next_id);
        curr_id = id;
    }
    spinlock.get(&target).unwrap().clone()
}

fn origin_neighbor_at(target: u32, skip: u32) -> u32 {
    let mut last: u32 = 0;
    let mut pos:  u32 = 0;
    for i in 0..target {
        let id = i + 1;
        pos = (pos + skip) % id;
        if pos == 0 {
            last = id;
        }
        pos += 1;
    }
    last
}


// ========== DATA ========================================

fn data() -> u32 {
    reader::to_word("./data/day17/input.txt")
        .parse::<u32>()
        .unwrap()
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_17() {
        assert_eq!(puzzle1(), 808);
        assert_eq!(puzzle2(), 47465686);
    }
}