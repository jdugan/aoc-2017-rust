use std::collections::HashMap;

use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    11
}

pub fn puzzle1() -> u32 {
    let steps  = data();
    final_distance(&steps)
}

pub fn puzzle2() -> u32 {
    let steps  = data();
    max_distance(&steps)
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn final_distance(steps: &Vec<String>) -> u32 {
    let     dxh = direction_deltas();
    let mut x   = 0_i32;
    let mut y   = 0_i32;
    for step in steps {
        let (dx, dy) = dxh.get(step).unwrap();
        x += dx;
        y += dy;
    }
    manhattan_distance(&x, &y)
}

fn max_distance(steps: &Vec<String>) -> u32 {
    let     dxh = direction_deltas();
    let mut max = 0;
    let mut x   = 0_i32;
    let mut y   = 0_i32;
    for step in steps {
        let (dx, dy) = dxh.get(step).unwrap();
        x += dx;
        y += dy;
        let md = manhattan_distance(&x, &y);
        if md > max {
            max = md;
        }
    }
    max
}


// ========== HELPERS =====================================

fn direction_deltas () -> HashMap<String, (i32, i32)> {
    let mut h: HashMap<String, (i32, i32)> = HashMap::new();
    h.insert("n".to_string(),  ( 0,  2));
    h.insert("nw".to_string(), (-1,  1));
    h.insert("ne".to_string(), ( 1,  1));
    h.insert("s".to_string(),  ( 0, -2));
    h.insert("sw".to_string(), (-1, -1));
    h.insert("se".to_string(), ( 1, -1));
    h
}

fn manhattan_distance(x: &i32, y: &i32) -> u32 {
    let ax = x.abs() as u32;
    let ay = y.abs() as u32;
    if ay > ax {
        (ay - ax)/2 + ax
    } else {
        ax
    }
}


// ========== DATA ========================================

fn data() -> Vec<String> {
    reader::to_word("./data/day11/input.txt")
        .split(",")
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
    fn test_day_11() {
        assert_eq!(puzzle1(), 764);
        assert_eq!(puzzle2(), 1532);
    }
}