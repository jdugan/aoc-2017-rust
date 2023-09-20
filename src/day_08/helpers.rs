use std::collections::HashMap;

// --------------------------------------------------------
// Command
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Command {
    pub do_register: String,
    pub do_action:   String,
    pub do_value:    i32,
    pub if_register: String,
    pub if_operator: String,
    pub if_value:    i32
}


// --------------------------------------------------------
// Command
// --------------------------------------------------------

#[derive(Debug)]
pub struct Computer {
    program: Vec<Command>,
    memory:  HashMap<String, i32>
}

impl Computer {
    // ========== CLASS METHODS ===========================

    pub fn new(program: &Vec<Command>) -> Computer {
        let mut c = Computer{ program: program.to_vec(), memory: HashMap::new() };
        c.initialize_memory();
        c
    }


    // ========== PUBLIC METHODS ==========================

    pub fn run(&mut self) -> Vec<HashMap<String, i32>> {
        let mut history = vec![];
        for cmd in self.program.clone() {
            if self.condition_satisfied(&cmd) {
                self.execute_command(&cmd);
                history.push(
                    self.memory.clone()
                );
            }
        }
        history
    }


    // ========== PRIVATE METHODS =========================

    fn initialize_memory(&mut self) {
        for cmd in &self.program {
            self.memory.insert(cmd.do_register.clone(), 0_i32);
            self.memory.insert(cmd.if_register.clone(), 0_i32);
        }
    }

    fn condition_satisfied(&self, cmd: &Command) -> bool {
        let mem_value = self.memory.get(&cmd.if_register).unwrap().clone();
        match cmd.if_operator.as_str() {
            ">"  => mem_value >  cmd.if_value,
            ">=" => mem_value >= cmd.if_value,
            "==" => mem_value == cmd.if_value,
            "!=" => mem_value != cmd.if_value,
            "<=" => mem_value <= cmd.if_value,
            "<"  => mem_value <  cmd.if_value,
            &_   => false
        }
    }

    fn execute_command(&mut self, cmd: &Command) {
        let mem_value = self.memory.get(&cmd.do_register).unwrap().clone();
        // let mut mem   = self.memory.clone();
        match cmd.do_action.as_str() {
            "inc" => self.memory.insert(cmd.do_register.clone(), mem_value + cmd.do_value),
            &_    => self.memory.insert(cmd.do_register.clone(), mem_value - cmd.do_value),
        };
    }
}