use std::collections::HashMap;

// ========== COMMAND =====================================

#[derive(Clone)]
#[derive(Debug)]
pub struct Command {
    pub do_register: String,
    pub do_action:   String,
    pub do_value:    i32,
    pub if_register: String,
    pub if_operator: String,
    pub if_value:    i32
}

impl Command {
    pub fn condition_satisfied(&self, memory: &HashMap<String, i32>) -> bool {
        let mem_value = memory.get(&self.if_register).unwrap().clone();
        match self.if_operator.as_str() {
            ">"  => mem_value >  self.if_value,
            ">=" => mem_value >= self.if_value,
            "==" => mem_value == self.if_value,
            "!=" => mem_value != self.if_value,
            "<=" => mem_value <= self.if_value,
            "<"  => mem_value <  self.if_value,
            &_   => false
        }
    }

    pub fn execute(&self, memory: &mut HashMap<String, i32>) -> HashMap<String, i32> {
        let mem_value = memory.get(&self.do_register).unwrap().clone();
        match self.do_action.as_str() {
            "inc" => memory.insert(self.do_register.clone(), mem_value + self.do_value),
            &_    => memory.insert(self.do_register.clone(), mem_value - self.do_value),
        };
        memory.clone()
    }
}


// ========== COMPUTER ====================================

#[derive(Debug)]
pub struct Computer {
    program: Vec<Command>,
    memory:  HashMap<String, i32>
}

impl Computer {
    pub fn new(program: &Vec<Command>) -> Computer {
        let mut c = Computer{ program: program.to_vec(), memory: HashMap::new() };
        c.initialize_memory();
        c
    }

    pub fn initialize_memory(&mut self) {
        for cmd in &self.program {
            self.memory.insert(cmd.do_register.clone(), 0_i32);
            self.memory.insert(cmd.if_register.clone(), 0_i32);
        }
    }

    pub fn run(&mut self) -> Vec<HashMap<String, i32>> {
        let mut history = vec![];
        for cmd in &self.program {
            if cmd.condition_satisfied(&self.memory) {
                history.push(cmd.execute(&mut self.memory));
            }
        }
        history
    }
}