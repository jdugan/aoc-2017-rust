pub mod helpers;

use crate::day_24::helpers::{ Pipe, Scenario };
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    24
}

pub fn puzzle1() -> u32 {
    let pipes     = data();
    let scenarios = find_scenarios(pipes);
    let scenario  = find_strongest(scenarios);
    scenario.strength()
}

pub fn puzzle2() -> u32 {
    let pipes     = data();
    let scenarios = find_scenarios(pipes);
    let longests  = find_longests(scenarios);
    let scenario  = find_strongest(longests);
    scenario.strength()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== SOLVERS =====================================

fn find_longests(scenarios: Vec<Scenario>) -> Vec<Scenario> {
    let max_length = scenarios.iter().map(|s| s.bridge.len()).max().unwrap();
    let longests   = scenarios.iter()
        .filter(|s| s.bridge.len() == max_length)
        .cloned()
        .collect::<Vec<Scenario>>();
    longests
}

fn find_scenarios(pipes: Vec<Pipe>) -> Vec<Scenario> {
    let mut scenarios: Vec<Scenario> = vec![];
    let mut possibles: Vec<Scenario> = vec![initial_scenario(pipes)];
    while possibles.len() > 0 {
        let mut new_possibles: Vec<Scenario> = vec![];
        for possible in possibles.iter() {
            let candidates = possible.candidate_pipes();
            if candidates.len() > 0 {
                for c in candidates {
                    let scenario = possible.add_pipe(c);
                    new_possibles.push(scenario);
                }
            } else {
                scenarios.push(possible.clone())
            }
        }
        possibles = new_possibles;
    }
    scenarios
}

fn find_strongest(mut scenarios: Vec<Scenario>) -> Scenario {
    scenarios.sort_by(|a, b| b.strength().cmp(&a.strength()));
    scenarios.into_iter().next().unwrap()
}

fn initial_scenario(pipes: Vec<Pipe>) -> Scenario {
    let bridge: Vec<Pipe> = vec![];
    let connector         = 0_u32;
    Scenario{ pipes, bridge, connector }
}


// ========== DATA ========================================

fn data() -> Vec<Pipe> {
    reader::to_strings("./data/day24/input.txt")
        .iter()
        .enumerate()
        .map(|(id, line)| {
            let mut parts = line.split("/");
            let left  = parts.next().unwrap().parse::<u32>().unwrap();
            let right = parts.next().unwrap().parse::<u32>().unwrap();
            Pipe{ id: id as u32, left, right }
        })
        .collect()
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_24() {
        assert_eq!(puzzle1(), 1511);
        assert_eq!(puzzle2(), 1471);
    }
}