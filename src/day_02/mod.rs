use crate::utility::reader;


// ----------------------------------------------------
// Public Methods
// ----------------------------------------------------

pub fn day() -> u8 {
    2
}

pub fn puzzle1() -> u32 {
    calculate_checksum()
}

pub fn puzzle2() -> u32 {
    calculate_distribution()
}


// ----------------------------------------------------
// Private Methods
// ----------------------------------------------------

// ========== SOLUTIONS ===============================

fn calculate_checksum() -> u32 {
    let spreadsheet  = data();
    let mut checksum = 0;
    for row in spreadsheet {
        let min   = row.iter().min().unwrap();
        let max   = row.iter().max().unwrap();
        checksum += max - min;
    }
    checksum
}

fn calculate_distribution() -> u32 {
    let spreadsheet      = data();
    let mut distribution = 0;
    'row: for mut row in spreadsheet {
        row.sort();
        let col_size = row.len();
        for i in 0..col_size-1 {
            let f0 = row[i];
            for j in i+1..col_size {
                let f1 = row[j];
                if f1 % f0 == 0 {
                    let dividend = f1 / f0;
                    distribution += dividend;
                    continue 'row;
                }
            }
        }
    }
    distribution
}


// ========== DATA ====================================

fn data() -> Vec<Vec<u32>> {
    reader::to_lines("./data/day02/input.txt")
        .into_iter()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}


// ----------------------------------------------------
// Unit Tests
// ----------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02() {
        assert_eq!(puzzle1(), 37923);
        assert_eq!(puzzle2(), 263);
    }
}