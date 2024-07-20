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
    memory:     HashMap<String, i64>,
    pointer:    i64,
    program:    Vec<Command>,
}

impl Computer {
    // ========== CLASS METHODS ===========================

    pub fn new(program: Vec<Command>) -> Computer {
        let mut c = Computer{
            memory:  HashMap::new(),
            pointer: 0,
            program
        };
        c.initialize_memory();
        c
    }


    // ========== PUBLIC METHODS ==========================

    pub fn shortcut(&self) -> u32 {
        let mut count = 0_u32;
        for f in 0..1001 {
            let n = 108400 + (17 * f);
            if self.is_not_prime(n) {
                count += 1;
            }
        }
        count
    }

    pub fn run(&mut self) -> u32 {
        let     cmds  = self.program.clone();
        let mut count = 0_u32;
        let mut valid = true;

        while valid {
            let cmd = &cmds[self.pointer as usize];
            valid   = self.execute_command(&cmd);
            if cmd.action.as_str() == "mul" {
                count += 1;
            }
        }
        count
    }


    // ========== PRIVATE (EXECUTION) =====================

    fn execute_command(&mut self, cmd: &Command) -> bool {
        match cmd.action.as_str() {
            "jnz" => self.execute_jnz(&cmd.register, &cmd.args),
            "mul" => self.execute_mul(&cmd.register, &cmd.args),
            "set" => self.execute_set(&cmd.register, &cmd.args),
            "sub" => self.execute_sub(&cmd.register, &cmd.args),
            &_    => {
                println!("Unrecognised command {} for register {} with args {:?}", cmd.action, cmd.register, cmd.args);
                false
            }
        }
    }

    fn execute_jnz(&mut self, register: &String, args: &Vec<String>) -> bool {
        let mv = self.evaluate_parameter(register);
        if mv != 0 {
            let v  = self.evaluate_parameter(&args[0]);
            self.pointer += v;
        } else {
            self.pointer += 1;
        }
        self.is_valid()
    }

    fn execute_mul(&mut self, register: &String, args: &Vec<String>) -> bool {
        let mv = self.evaluate_parameter(register);
        let v  = self.evaluate_parameter(&args[0]);
        self.memory.insert(register.clone(), mv * v);
        self.advance_and_check_validity()
    }

    fn execute_set(&mut self, register: &String, args: &Vec<String>) -> bool {
        let v = self.evaluate_parameter(&args[0]);
        self.memory.insert(register.clone(), v);
        self.advance_and_check_validity()
    }

    fn execute_sub(&mut self, register: &String, args: &Vec<String>) -> bool {
        let mv = self.evaluate_parameter(register);
        let v  = self.evaluate_parameter(&args[0]);
        self.memory.insert(register.clone(), mv - v);
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

    fn initialize_memory(&mut self) {
        self.memory.insert(String::from("a"), 0);
        self.memory.insert(String::from("b"), 0);
        self.memory.insert(String::from("c"), 0);
        self.memory.insert(String::from("d"), 0);
        self.memory.insert(String::from("e"), 0);
        self.memory.insert(String::from("f"), 0);
        self.memory.insert(String::from("g"), 0);
        self.memory.insert(String::from("h"), 0);
    }

    fn is_not_prime(&self, n: usize) -> bool {
        let mut found = false;
        let mut f     = 2;
        while f * f < n {
            if n % f == 0 {
                found = true;
                break
            }
            f += 1;
        }
        found
    }

    fn is_valid(&self) -> bool {
        let max = self.program.len() as i64;
        self.pointer >= 0 && self.pointer < max
    }
}