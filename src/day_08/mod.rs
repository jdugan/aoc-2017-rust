pub mod helpers;

use crate::day_08::helpers::{ Command, Computer };
use crate::utility::{ converter, reader };


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
        let cmd_parts = converter::string_to_words(&cmd_str);
        let cmd = Command{
            do_register: cmd_parts[0].clone(),
            do_action:   cmd_parts[1].clone(),
            do_value:    cmd_parts[2].parse::<i32>().unwrap(),
            if_register: cmd_parts[3].clone(),
            if_operator: cmd_parts[4].clone(),
            if_value:    cmd_parts[5].parse::<i32>().unwrap(),
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