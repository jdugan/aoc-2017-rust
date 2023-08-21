use std::collections::HashMap;
use std::collections::HashSet;

use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    12
}

pub fn puzzle1() -> usize {
    let network  = data();
    let groups   = build_groups(&network);
    let root_key = 0_u32;
    groups
        .iter()
        .filter(|g| g.contains(&root_key))
        .map(|g| g.len())
        .sum()
}

pub fn puzzle2() -> usize {
    let network = data();
    let groups  = build_groups(&network);
    groups.len()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== HELPERS =====================================

fn build_groups(network: &HashMap<u32, Vec<u32>>) -> Vec<Vec<u32>> {
    let mut groups: Vec<Vec<u32>> = vec![];
    for (id, _) in network {
        groups.push(generate_group(network, id));
    }
    groups.sort_unstable();
    groups.dedup();
    groups
}

fn generate_group(network: &HashMap<u32, Vec<u32>>, key: &u32) -> Vec<u32> {
    //  find unique keys
    let mut check_keys: HashSet<u32> = HashSet::new();
    let mut found_keys: HashSet<u32> = HashSet::new();
    check_keys.insert(key.clone());
    found_keys.insert(key.clone());
    while check_keys.len() > 0 {
        let loop_keys = check_keys.clone();
        check_keys = HashSet::new();
        for key in loop_keys {
            let kids = network.get(&key).unwrap();
            for kid in kids {
                if found_keys.insert(kid.clone()) {
                    check_keys.insert(kid.clone());
                }
            }
        }
    }
    // organise into a sorted vector
    let mut group = Vec::from_iter(found_keys);
    group.sort_unstable();
    group
}


// ========== DATA ========================================

fn data() -> HashMap<u32, Vec<u32>> {
    reader::to_strings("./data/day12/input.txt")
        .iter()
        .map(|line| parse_line(line))
        .fold(HashMap::new(), |mut hash, (k, v)| {
            hash.insert(k, v);
            hash
        })
}

fn parse_line(line: &String) -> (u32, Vec<u32>) {
    let mut parts = line.split(" <-> ");
    let key       = parts.next().unwrap().parse::<u32>().unwrap();
    let s_list    = parts.next().unwrap();
    let v_list    = s_list.split(", ").map(|s| s.parse::<u32>().unwrap()).collect();
    (key, v_list)
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_12() {
        assert_eq!(puzzle1(), 113);
        assert_eq!(puzzle2(), 202);
    }
}