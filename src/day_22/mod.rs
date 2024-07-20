pub mod helpers;

use std::collections::HashMap;

use crate::day_22::helpers::Virus;
use crate::utility::{ converter, reader };


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    22
}

pub fn puzzle1() -> usize {
    let mut points = data();
    let mut virus  = Virus::new();
    for _ in 0..10_000 {
        points = virus.simple_burst(points);
    }
    virus.infection_count
}

pub fn puzzle2() -> usize {
    let mut points = data();
    let mut virus  = Virus::new();
    for _ in 0..10_000_000 {
        points = virus.evolved_burst(points);
    }
    virus.infection_count
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== UTILITY =====================================

// fn print(points: &HashMap<(i32, i32), String>) {
//     let mut rows: Vec<String> = vec![];
//     let mut max_x = -1_000_000_000_i32;
//     let mut min_x = 1_000_000_000_i32;
//     let mut max_y = -1_000_000_000_i32;
//     let mut min_y = 1_000_000_000_i32;
//     for (x, y) in points.keys() {
//         if *x > max_x { max_x = *x };
//         if *x < min_x { min_x = *x };
//         if *y > max_y { max_y = *y };
//         if *y < min_y { min_y = *y };
//     }
//     println!("{:?}", points);
//     println!("{:?}", (min_x, max_x));
//     println!("{:?}", (min_y, max_y));
//     for y in min_y..max_y+1 {
//         let mut row = String::from("");
//         for x in min_x..max_x+1 {
//             let id = (x, y);
//             match points.get(&id) {
//                 Some(s) => row += s,
//                 None    => row += "."
//             }
//         }
//         rows.push(row);
//     }
//     rows.reverse();
//     println!("");
//     for row in rows {
//         println!("{:?}", row)
//     }
//     println!("");
// }


// ========== DATA ========================================

fn data() -> HashMap<(i32, i32), String> {
    let mut points: HashMap<(i32, i32), String> = HashMap::new();
    let mut lines = reader::to_strings("./data/day22/input.txt");
    let size      = lines.len();
    let ox        = (size / 2) as i32;
    let oy        = (size / 2) as i32;
    lines.reverse();
    for (uy, row) in lines.iter().enumerate() {
        for (ux, value) in converter::string_to_letters(row).iter().enumerate() {
            if value == "#" {
                let x = (ux as i32) - ox;
                let y = (uy as i32) - oy;
                points.insert((x, y), String::from("#"));
            }
        }
    }
    points
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_22() {
        assert_eq!(puzzle1(), 5280);
        assert_eq!(puzzle2(), 2512261);
    }
}