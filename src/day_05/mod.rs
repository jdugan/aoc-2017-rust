use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    5
}

pub fn puzzle1() -> u32 {
    let mut maze   = data();
    let upper: i32 = 1;
    let lower: i32 = 1;

    escape_steps(&mut maze, upper, lower)
}

pub fn puzzle2() -> u32 {
    let mut maze   = data();
    let upper: i32 = -1;
    let lower: i32 = 1;

    escape_steps(&mut maze, upper, lower)
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

// This has slightly more complicated logic than would
// be needed if we used a hashmap, but it is significantly
// faster.
//
fn escape_steps(maze: &mut Vec<i32>, upper_increment: i32, lower_increment: i32) -> u32 {
    let mut steps = 0;
    let mut pos   = 0;
    let     max   = maze.len();
    loop {
        match maze.get(pos).copied() {
            Some(offset) => {
                if offset > 2 {
                    maze[pos] += upper_increment;
                } else {
                    maze[pos] += lower_increment;
                }
                let offabs = offset.abs() as usize;
                match offset {
                    n if n < 0i32 && offabs > pos       => break,         // error :: overflows left
                    n if n > 0i32 && pos + offabs > max => break,         // error :: overflows right
                    n if n < 0i32                       => pos -= offabs, // moves safely left
                    _                                   => pos += offabs, // moves safely right
                }
            },
            None => break,
        }
        steps += 1;
    }
    steps
}


// ========== DATA ========================================

fn data() -> Vec<i32> {
    reader::to_signed_ints("./data/day05/input.txt")
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_00() {
        assert_eq!(puzzle1(), 394829);
        assert_eq!(puzzle2(), 31150702);
    }
}