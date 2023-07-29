pub mod day_01;
use crate::day_01::{
    day,
    puzzle1,
    puzzle2
};

fn main() {
    println!("");
    println!("DAY {}:", day());
    println!("  Puzzle 1 => {0}", puzzle1());
    println!("  Puzzle 2 => {0}", puzzle2());
    println!("");
}