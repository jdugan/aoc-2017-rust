use crate::reader::{
    read_chars
};


// ----------------------------------------------------
// Public Methods
// ----------------------------------------------------

pub fn day() -> u8 {
    1
}

pub fn puzzle1() -> u32 {
    let digits = data();
    let step   = 1;
    solve_captcha(digits, step)
}

pub fn puzzle2() -> u32 {
    let digits = data();
    let step   = digits.len() / 2;
    solve_captcha(digits, step)
}


// ----------------------------------------------------
// Private Methods
// ----------------------------------------------------

// ========== SOULTIONS ===============================

fn solve_captcha(digits: Vec<u32>, step: usize) -> u32 {
    let count   = digits.len();
    let mut sum = 0;
    for (i, d) in digits.iter().enumerate() {
        let d0 = *d;
        let d1 = digits[(i + step) % count];
        if d0 == d1 {
            sum += d0;
        }
    }
    sum
}


// ========== DATA ====================================

fn data() -> Vec<u32> {
    read_chars("./data/day01/input.txt")
        .into_iter()
        .map(String::from)
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}


// ----------------------------------------------------
// Unit Tests
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01() {
        assert_eq!(puzzle1(), 1141);
        assert_eq!(puzzle2(), 950);
    }
}