use std::collections::HashMap;

use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    13
}

pub fn puzzle1() -> u32 {
    let (firewall, min, max) = data();
    let offset               = 0_u32;
    let (severity, _) = transit(&firewall, &min, &max, &offset, false);
    severity
}

pub fn puzzle2() -> u32 {
    let (firewall, min, max) = data();
    let mut offset           = 1_u32;
    loop {
        let (_, count) = transit(&firewall, &min, &max, &offset, true);
        if count == 0 {
            break;
        } else {
            offset += 1;
        }
    }
    offset
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn transit(firewall: &HashMap<u32, u32>, min: &u32, max: &u32, offset: &u32, breakable: bool) -> (u32, u32) {
    let mut severity = 0_u32;
    let mut count    = 0_u32;
    for range in min.clone()..max.clone() {
         match firewall.get(&range) {
            Some(depth) => {
                let dist = 2 * (depth - 1);
                if (range + offset) % dist == 0 {
                    severity += range * depth;
                    count    += 1;
                    if breakable {
                        break;
                    }
                }
            },
            None => continue,
        }
    }
    (severity, count)
}


// ========== DATA ========================================

fn data() -> (HashMap<u32, u32>, u32, u32) {
    let firewall =
        reader::to_strings("./data/day13/input.txt")
            .iter()
            .map(|line| parse_line(line))
            .fold(HashMap::new(), |mut hash, (k, v)| {
                hash.insert(k, v);
                hash
            });
    let min = 0_u32;
    let max = firewall.keys().max().unwrap().clone() + 1;
    (firewall, min, max)
}

fn parse_line(line: &String) -> (u32, u32) {
    let mut parts = line.split(": ");
    let depth     = parts.next().unwrap().parse::<u32>().unwrap();
    let range     = parts.next().unwrap().parse::<u32>().unwrap();
    (depth, range)
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_13() {
        assert_eq!(puzzle1(), 1904);
        assert_eq!(puzzle2(), 3833504);
    }
}