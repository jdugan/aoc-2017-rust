use std::collections::HashMap;

// --------------------------------------------------------
// Command
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Command {
    pub action:   String,
    pub register: String,
    pub args:     Vec<String>,
}


// --------------------------------------------------------
// Computer
// --------------------------------------------------------

#[derive(Debug)]
pub struct Computer {
    pub input:  Vec<i64>,
    pub output: Vec<i64>,
    memory:     HashMap<String, i64>,
    pointer:    i64,
    program:    Vec<Command>,
}

impl Computer {
    // ========== CLASS METHODS ===========================

    pub fn new(id: i64, program: Vec<Command>) -> Computer {
        let mut c = Computer{
            input:   vec![],
            output:  vec![],
            memory:  HashMap::new(),
            pointer: 0,
            program
        };
        c.initialize_memory(id);
        c
    }


    // ========== PUBLIC METHODS ==========================

    pub fn is_deadlocked(&self) -> bool {
        self.input.is_empty() && self.output.is_empty()
    }

    pub fn run(&mut self) -> u64 {
        let     cmds  = self.program.clone();
        let mut count = 0_u64;
        let mut valid = true;

        while valid {
            let cmd = &cmds[self.pointer as usize];
            valid   = self.execute_command(&cmd);
            count  += 1;
        }
        count
    }


    // ========== PRIVATE (EXECUTION) =====================

    fn execute_command(&mut self, cmd: &Command) -> bool {
        match cmd.action.as_str() {
            "add" => self.execute_add(&cmd.register, &cmd.args),
            "jgz" => self.execute_jgz(&cmd.register, &cmd.args),
            "mod" => self.execute_mod(&cmd.register, &cmd.args),
            "mul" => self.execute_mul(&cmd.register, &cmd.args),
            "rcv" => self.execute_rcv(&cmd.register),
            "set" => self.execute_set(&cmd.register, &cmd.args),
            "snd" => self.execute_snd(&cmd.register),
            &_    => {
                println!("Unrecognised command {} for register {} with args {:?}", cmd.action, cmd.register, cmd.args);
                false
            }
        }
    }

    fn execute_add(&mut self, register: &String, args: &Vec<String>) -> bool {
        let mv = self.evaluate_parameter(register);
        let v  = self.evaluate_parameter(&args[0]);
        self.memory.insert(register.clone(), mv + v);
        self.advance_and_check_validity()
    }

    fn execute_jgz(&mut self, register: &String, args: &Vec<String>) -> bool {
        let mv = self.evaluate_parameter(register);
        if mv > 0 {
            let v  = self.evaluate_parameter(&args[0]);
            self.pointer += v;
        } else {
            self.pointer += 1;
        }
        self.is_valid()
    }

    fn execute_mod(&mut self, register: &String, args: &Vec<String>) -> bool {
        let mv = self.evaluate_parameter(register);
        let v  = self.evaluate_parameter(&args[0]);
        self.memory.insert(register.clone(), mv % v);
        self.advance_and_check_validity()
    }

    fn execute_mul(&mut self, register: &String, args: &Vec<String>) -> bool {
        let mv = self.evaluate_parameter(register);
        let v  = self.evaluate_parameter(&args[0]);
        self.memory.insert(register.clone(), mv * v);
        self.advance_and_check_validity()
    }

    fn execute_rcv(&mut self, register: &String) -> bool {
        if self.input.len() > 0 {
            let iv = self.input.remove(0);
            self.memory.insert(register.clone(), iv);
            self.advance_and_check_validity()
        } else {
            // println!("Progam {} halting :: input queue empty", self.id);
            false
        }
    }

    fn execute_set(&mut self, register: &String, args: &Vec<String>) -> bool {
        let v = self.evaluate_parameter(&args[0]);
        self.memory.insert(register.clone(), v);
        self.advance_and_check_validity()
    }

    fn execute_snd(&mut self, register: &String) -> bool {
        let v = self.evaluate_parameter(register);
        self.output.push(v);
        self.advance_and_check_validity()
    }


    // ========== PRIVATE (HELPERS) =======================

    fn advance_and_check_validity(&mut self) -> bool {
        self.pointer += 1;
        self.is_valid()
    }

    fn evaluate_parameter(&self, param: &String) -> i64 {
        let v = param.parse::<i64>();
        match v {
            Ok(i)  => i,
            Err(_) => self.memory.get(param).unwrap().clone()
        }
    }

    fn initialize_memory(&mut self, id: i64) {
        for cmd in &self.program {
            self.memory.insert(cmd.register.clone(), 0);
        }
        self.memory.insert("p".to_string(), id);
    }

    fn is_valid(&self) -> bool {
        let max = self.program.len() as i64;
        self.pointer >= 0 && self.pointer < max
    }
}