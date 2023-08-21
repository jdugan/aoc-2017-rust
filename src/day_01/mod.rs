use crate::utility::{ converter, reader };


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    1
}

pub fn puzzle1() -> u32 {
    let digits = data();
    let step   = 1;
    solve_captcha(&digits, &step)
}

pub fn puzzle2() -> u32 {
    let digits = data();
    let step   = &digits.len() / 2;
    solve_captcha(&digits, &step)
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn solve_captcha(digits: &Vec<u32>, step: &usize) -> u32 {
    let count = digits.len();
    digits.iter().enumerate().fold(0_u32, |sum, (i, d)| {
        let d0 = *d;
        let d1 = digits[(i + step) % count];
        match d0 == d1 {
            true  => sum + d0,
            false => sum,
        }
    })
}


// ========== DATA ========================================

fn data() -> Vec<u32> {
    let word = reader::to_strings("./data/day01/input.txt")[0].clone();
    converter::string_to_letters(&word)
        .iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01() {
        assert_eq!(puzzle1(), 1141);
        assert_eq!(puzzle2(), 950);
    }
}