// --------------------------------------------------------
// Configuration
// --------------------------------------------------------

// declare modules
pub mod common;
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod utility;

// import daily solution
use crate::day_07::{
    day,
    puzzle1,
    puzzle2
};


// --------------------------------------------------------
// Main
// --------------------------------------------------------

fn main() {
    println!("");
    println!("DAY {}:", day());
    println!("  Puzzle 1 => {0}", puzzle1());
    println!("  Puzzle 2 => {0}", puzzle2());
    println!("");
}