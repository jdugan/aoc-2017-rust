use std::collections::HashMap;

// --------------------------------------------------------
// Machine
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Machine {
    pub position: i32,
    pub state:    String,
    pub rules:    HashMap<String, Rule>,
    pub memory:   HashMap<i32, bool>
}

impl Machine {
    // ========== CLASS METHODS ===========================

    pub fn new(rules: HashMap<String, Rule>, state: String) -> Machine {
        Machine {
            memory: HashMap::new(),
            position: 0_i32,
            rules: rules,
            state: state
        }
    }

    // ========== PUBLIC METHODS ==========================

    pub fn checksum(&self) -> usize {
        self.memory.keys().len()
    }

    pub fn run(&mut self, steps: u32) {
        for _ in 0..steps {
            let rule = self.rules.get(&self.state).unwrap();
            match self.memory.contains_key(&self.position) {
                true => {
                    match rule.on_value {
                        1_u32 => self.memory.insert(self.position, true),
                        _     => self.memory.remove(&self.position)
                    };
                    self.position += rule.on_increment;
                    self.state     = rule.on_state.clone();
                },
                false => {
                    match rule.off_value {
                        1_u32 => self.memory.insert(self.position, true),
                        _     => self.memory.remove(&self.position)
                    };
                    self.position += rule.off_increment;
                    self.state     = rule.off_state.clone();
                }
            }
        }
    }
}


// --------------------------------------------------------
// Rule
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Rule {
    pub id:             String,
    pub off_value:      u32,
    pub off_increment:  i32,
    pub off_state:      String,
    pub on_value:       u32,
    pub on_increment:   i32,
    pub on_state:       String,
}