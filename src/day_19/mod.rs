use std::collections::HashMap;

use crate::common::grid::Point;
use crate::utility::converter;
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    19
}

pub fn puzzle1() -> String {
    let points    = data();
    let (path, _) = find_path(&points);
    path
}

pub fn puzzle2() -> u32 {
    let points    = data();
    let (_, dist) = find_path(&points);
    dist
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------


// ========== SOLVERS =====================================

fn find_path(points: &HashMap<Point, String>) -> (String, u32) {
    let mut path      = "".to_string();
    let mut direction = "south";
    let mut current   = find_origin(&points);
    let mut next      = find_next(&points, &current, &direction);
    let mut distance  = 1_u32;
    while current != next {
        let display = points.get(&next).unwrap().as_str();
        if display >= "A" && display <= "Z" {
            path += display;
        }
        if display == "+" {
            if direction == "north" || direction == "south" {
                let (x, y) = next.east_id();
                let p      = Point::new(x, y);
                direction  = match points.get(&p) {
                    Some(_) => "east",
                    None    => "west",
                }
            } else {
                let (x, y) = next.north_id();
                let p      = Point::new(x, y);
                direction  = match points.get(&p) {
                    Some(_) => "north",
                    None    => "south",
                }
            }
        }
        distance += 1;
        current   = next;
        next      = find_next(&points, &current, &direction);
    }
    (path, distance)
}


// ========== HELPERS =====================================

fn find_next(points: &HashMap<Point, String>, current: &Point, direction: &str) -> Point {
    let (x, y) = match direction {
        "east"  => current.east_id(),
        "north" => current.north_id(),
        "south" => current.south_id(),
        _       => current.west_id(),
    };
    let p = Point::new(x, y);
    match points.get(&p) {
        Some(_) => p,
        None    => Point::new(current.x, current.y),
    }
}

fn find_origin(points: &HashMap<Point, String>) -> Point {
    let p = points.keys().find(|&p| p.y == 0).unwrap();
    Point::new(p.x, p.y)
}


// ========== DATA ========================================

fn data() -> HashMap<Point, String> {
    let mut hm: HashMap<Point, String> = HashMap::new();
    let rows = reader::to_strings("./data/day19/input.txt");
    for (y, row) in rows.iter().enumerate() {
        let letters = converter::string_to_letters(row);
        for (x, letter) in letters.into_iter().enumerate() {
            if letter.as_str() != " " {
                let iy = -1 * (y as i32);
                hm.insert(Point::new(x as i32, iy as i32), letter);
            }
        }
    }
    hm
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_19() {
        assert_eq!(puzzle1(), "QPRYCIOLU".to_string());
        assert_eq!(puzzle2(), 16162);
    }
}