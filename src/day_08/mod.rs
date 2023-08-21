pub mod helpers;

use crate::day_08::helpers::{ Command, Computer };
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    8
}

pub fn puzzle1() -> i32 {
    let program      = commands();
    let mut computer = Computer::new(&program);
    let mut history  = computer.run();

    match history.pop() {
        Some(mem) => mem.into_values().max().unwrap(),
        None      => -1
    }
}

pub fn puzzle2() -> i32 {
    let program      = commands();
    let mut computer = Computer::new(&program);
    let history      = computer.run();

    history
        .into_iter()
        .map(|mem| mem.into_values().max().unwrap())
        .max()
        .unwrap()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== DATA ====================================

fn data() -> Vec<String> {
    reader::to_strings("./data/day08/input.txt")
}

fn commands() -> Vec<Command> {
    let mut cmds: Vec<Command> = vec![];
    for line in  data() {
        let cmd_str   = line.replace("if ", "");
        let mut parts = cmd_str.split(" ");
        let cmd = Command{
            do_register: parts.next().unwrap().to_string(),
            do_action:   parts.next().unwrap().to_string(),
            do_value:    parts.next().unwrap().parse::<i32>().unwrap(),
            if_register: parts.next().unwrap().to_string(),
            if_operator: parts.next().unwrap().to_string(),
            if_value:    parts.next().unwrap().parse::<i32>().unwrap(),
        };
        cmds.push(cmd);
    }
    cmds
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_08() {
        assert_eq!(puzzle1(), 5075);
        assert_eq!(puzzle2(), 7310);
    }
}