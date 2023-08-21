use std::collections::HashMap;

use crate::common::grid::Point;
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    3
}

pub fn puzzle1() -> u32 {
    let target = data();
    distance_to_ordinal(target)
}

pub fn puzzle2() -> u32 {
    let target = data();
    stress_test_result(target)
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn stress_test_result(target: u32) -> u32 {
    let mut grid    = HashMap::new();
    let mut point   = Point::origin();
    let mut ordinal = 1;
    let mut value   = 1;
    grid.insert(point.id(), value);

    'outer: loop {
        let steps = f32::sqrt(ordinal as f32) as u32;
        let mut x;
        let mut y;

        //  move east
        (x, y)   = point.east_id();
        point    = Point::new(x, y);
        ordinal += 1;
        value    = neighborhood_sum(&grid, &point);
        if value > target {
            break 'outer;
        }
        grid.insert(point.id(), value);

        // move north
        for _ in 0..steps {
            (x, y)   = point.north_id();
            point    = Point::new(x, y);
            ordinal += 1;
            value    = neighborhood_sum(&grid, &point);
            if value > target {
                break 'outer;
            }
            grid.insert(point.id(), value);
        }

        // move west
        for _ in 0..steps+1 {
            (x, y)   = point.west_id();
            point    = Point::new(x, y);
            ordinal += 1;
            value    = neighborhood_sum(&grid, &point);
            if value > target {
                break 'outer;
            }
            grid.insert(point.id(), value);
        }

        // move south
        for _ in 0..steps+1 {
            (x, y)   = point.south_id();
            point    = Point::new(x, y);
            ordinal += 1;
            value    = neighborhood_sum(&grid, &point);
            if value > target {
                break 'outer;
            }
            grid.insert(point.id(), value);
        }

        // move east
        for _ in 0..steps+1 {
            (x, y)   = point.east_id();
            point    = Point::new(x, y);
            ordinal += 1;
            value    = neighborhood_sum(&grid, &point);
            if value > target {
                break 'outer;
            }
            grid.insert(point.id(), value);
        }
    }

    value
}

fn distance_to_ordinal(target: u32) -> u32 {
    let mut point   = Point::origin();
    let mut ordinal = 1;

    'outer: loop {
        let steps = f32::sqrt(ordinal as f32) as u32;
        let mut x;
        let mut y;

        //  move east
        (x, y)   = point.east_id();
        point    = Point::new(x, y);
        ordinal += 1;
        if ordinal == target {
            break 'outer;
        }

        // move north
        for _ in 0..steps {
            (x, y)   = point.north_id();
            point    = Point::new(x, y);
            ordinal += 1;
            if ordinal == target {
                break 'outer;
            }
        }

        // move west
        for _ in 0..steps+1 {
            (x, y)   = point.west_id();
            point    = Point::new(x, y);
            ordinal += 1;
            if ordinal == target {
                break 'outer;
            }
        }

        // move south
        for _ in 0..steps+1 {
            (x, y)   = point.south_id();
            point    = Point::new(x, y);
            ordinal += 1;
            if ordinal == target {
                break 'outer;
            }
        }

        // move east
        for _ in 0..steps+1 {
            (x, y)   = point.east_id();
            point    = Point::new(x, y);
            ordinal += 1;
            if ordinal == target {
                break 'outer;
            }
        }
    }

    point.manhatten_distance()
}


// ========== HELPERS =====================================

fn neighborhood_sum(grid: &HashMap<(i32, i32), u32>, point: &Point) -> u32 {
    let mut sum: u32 = 0;
    for id in point.adjacent_ids() {
        match grid.get(&id) {
            Some(&value) => sum += value,
            _            => sum += 0
        }
    }
    sum
}


// ========== DATA ========================================

fn data() -> u32 {
    reader::to_word("./data/day03/input.txt")
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
    fn test_day_03() {
        assert_eq!(puzzle1(), 475);
        assert_eq!(puzzle2(), 279138);
    }
}