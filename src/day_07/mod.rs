use std::collections::{ HashMap, HashSet };

use crate::utility::reader;


// --------------------------------------------------------
// Strucutures
// --------------------------------------------------------

#[derive(Debug)]
struct Tower {
    weight:      u32,
    child_names: Vec<String>
}


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    7
}

pub fn puzzle1() -> String {
    let towers = towers();
    root_name(&towers)
}

pub fn puzzle2() -> u32 {
    let towers = towers();
    weight_variance(&towers)
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn root_name(towers: &HashMap<String, Tower>) -> String {
    let mut keys: HashSet<String> = HashSet::new();
    let mut kids: HashSet<String> = HashSet::new();
    for (k, t) in towers {
        keys.insert(k.to_string());
        for cn in &t.child_names {
            kids.insert(cn.to_string());
        }
    }
    keys.symmetric_difference(&kids).next().unwrap().to_string()
}

fn weight_variance(towers: &HashMap<String, Tower>) -> u32 {
    let mut tower_key  = root_name(&towers);
    let mut tower_diff = 0_u32;
    loop {
        let (key, diff) = inspect_imbalance(&towers, &tower_key);
        if diff != 0 {
            tower_key  = key;
            tower_diff = diff;
        } else {
            break;
        }
    }
    let tower = towers.get(&tower_key).unwrap();
    tower.weight - tower_diff
}


// ========== HELPERS =====================================

fn aggregate_weight(towers: &HashMap<String, Tower>, key: &String) -> u32 {
    let     tower  = towers.get(key).unwrap();
    let mut weight = tower.weight;
    for cn in &tower.child_names {
        weight += aggregate_weight(&towers, cn);
    }
    weight
}

fn inspect_imbalance(towers: &HashMap<String, Tower>, key: &String) -> (String, u32) {
    let     tower = towers.get(key).unwrap();
    let mut results: Vec<(String, u32)> = vec![];
    let mut imbalanced_key  = key.clone().to_string();
    let mut imbalanced_diff = 0;

    if tower.child_names.len() > 0 {
        // get tower weights
        for cn in &tower.child_names {
            results.push((cn.to_string(), aggregate_weight(towers, cn)));
        }
        // find ideal weight
        let mut weights: HashSet<u32> = HashSet::new();
        let mut ideal: u32 = 0;
        for (_, w) in &results {
            if !weights.insert(*w) {
                ideal = *w;
                break;
            }
        }
        // identify imbalanced tower and diff
        for (k, w) in &results {
            if *w != ideal {
                imbalanced_key  = k.to_string();
                imbalanced_diff = *w - ideal;
                break;
            }
        }
    }
    (imbalanced_key, imbalanced_diff)
}




// ========== DATA ====================================

fn data() -> Vec<String> {
    reader::to_strings("./data/day07/input.txt")
}

fn towers() -> HashMap<String, Tower> {
    let mut towers = HashMap::new();
    for line in data() {
        let mut outers = line.split(" -> ");
        let mut lefts  = outers.next().unwrap().split(" (");
        let     rights = match outers.next() {
            Some(list) => list.split(", ").map(String::from).collect(),
            None       => vec![]
        };
        let name        = lefts.next().unwrap().to_string();
        let weight      = lefts.next().unwrap().replace(")", "").parse::<u32>().unwrap();
        towers.insert(name, Tower{ weight, child_names: rights });
    }
    towers
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_07() {
        assert_eq!(puzzle1(), "fbgguv");
        assert_eq!(puzzle2(), 1864);
    }
}