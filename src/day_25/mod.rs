pub mod helpers;

use std::collections::HashMap;

use crate::day_25::helpers::{ Machine, Rule };

// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    25
}

pub fn puzzle1() -> usize {
    let (mut machine, steps) = data();
    machine.run(steps);
    machine.checksum()
}

pub fn puzzle2() -> u32 {
    50
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== DATA ========================================
// too lazy to parse this mess. way easier to just create
// the data structures by hand.
// --------------------------------------------------------

fn data() -> (Machine, u32) {
    let rule_a = Rule{
        id:             String::from("A"),
        off_value:      1_u32,
        off_increment:  1_i32,
        off_state:      String::from("B"),
        on_value:       0_u32,
        on_increment:   -1_i32,
        on_state:       String::from("C"),
    };
    let rule_b = Rule{
        id:             String::from("B"),
        off_value:      1_u32,
        off_increment:  -1_i32,
        off_state:      String::from("A"),
        on_value:       1_u32,
        on_increment:   1_i32,
        on_state:       String::from("D"),
    };
    let rule_c = Rule{
        id:             String::from("C"),
        off_value:      0_u32,
        off_increment:  -1_i32,
        off_state:      String::from("B"),
        on_value:       0_u32,
        on_increment:   -1_i32,
        on_state:       String::from("E"),
    };
    let rule_d = Rule{
        id:             String::from("D"),
        off_value:      1_u32,
        off_increment:  1_i32,
        off_state:      String::from("A"),
        on_value:       0_u32,
        on_increment:   1_i32,
        on_state:       String::from("B"),
    };
    let rule_e = Rule{
        id:             String::from("E"),
        off_value:      1_u32,
        off_increment:  -1_i32,
        off_state:      String::from("F"),
        on_value:       1_u32,
        on_increment:   -1_i32,
        on_state:       String::from("C"),
    };
    let rule_f = Rule{
        id:             String::from("F"),
        off_value:      1_u32,
        off_increment:  1_i32,
        off_state:      String::from("D"),
        on_value:       1_u32,
        on_increment:   1_i32,
        on_state:       String::from("A"),
    };

    let mut rules = HashMap::new();
    rules.insert(String::from("A"), rule_a);
    rules.insert(String::from("B"), rule_b);
    rules.insert(String::from("C"), rule_c);
    rules.insert(String::from("D"), rule_d);
    rules.insert(String::from("E"), rule_e);
    rules.insert(String::from("F"), rule_f);
    let state     = String::from("A");
    let steps     = 12481997_u32;

    (Machine::new(rules, state), steps)
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_25() {
        assert_eq!(puzzle1(), 3362);
        assert_eq!(puzzle2(), 50);
    }
}