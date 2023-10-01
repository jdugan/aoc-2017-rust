use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    2
}

pub fn puzzle1() -> u32 {
    calculate_checksum()
}

pub fn puzzle2() -> u32 {
    calculate_distribution()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn calculate_checksum() -> u32 {
    let spreadsheet = data();
    spreadsheet.iter().fold(0_u32, |sum, row| {
        let min = row.iter().min().unwrap();
        let max = row.iter().max().unwrap();
        sum + max - min
    })
}

fn calculate_distribution() -> u32 {
    let spreadsheet = data();
    spreadsheet.iter().fold(0_u32, |sum, r| {
        let     len = r.len();
        let mut row = r.clone();
        row.sort_unstable();

        sum + row.iter().enumerate().fold(0_u32, |quot, (i, v)| {
            match quot > 0 {
                true  => quot,      // stop looking as soon as we find a value
                false => {          // slightly inefficient but more readable for these lengths
                    (i+1..len).fold(0_u32, |rsum, j| {
                        match row[j] % v == 0 {
                            true  => rsum + (row[j] / v),
                            false => rsum
                        }
                    })
                }
            }
        })
    })
}


// ========== DATA ========================================

fn data() -> Vec<Vec<u32>> {
    reader::to_strings("./data/day02/input.txt")
        .iter()
        .map(|line| parse_line(line))
        .collect()
}

fn parse_line(line: &String) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02() {
        assert_eq!(puzzle1(), 37923);
        assert_eq!(puzzle2(), 263);
    }
}