use std::collections::HashMap;

#[derive(Debug)]
pub struct Party {
    pub size:      u8,
    pub dancers:   HashMap<u8, String>,
    pub positions: HashMap<String, u8>,
    pub moves:     Vec<String>,
}

impl Party {
    // ========== CLASS METHODS ===========================

    pub fn new(size: u8, moves: Vec<String>) -> Party {
        let mut dancers:   HashMap<u8, String> = HashMap::new();
        let mut positions: HashMap<String, u8> = HashMap::new();
        for i in 0..size {
            let name = (97_u8 + i) as char;
            dancers.insert(i, name.to_string());
            positions.insert(name.to_string(), i);
        }
        Party{ size, dancers, positions, moves }
    }


    // ========== PUBLIC METHODS ==========================

    pub fn dance(&mut self) -> String {
        for m in self.moves.clone() {
            let cmd: String       = m[..1].to_string();
            let args: Vec<String> = m[1..].split("/").map(String::from).collect();
            match cmd.as_str() {
                "x" => self.exchange(args),
                "p" => self.partner(args),
                "s" => self.spin(args),
                _   => println!("Unknown cmd {:?} with args {:?}", cmd, args)
            }
        }
        self.state()
    }

    pub fn dance_all_night(&mut self) -> String {
        let (steps, period) = self.choreograph();
        let time_index      = 1000000000_u64;
        let offset          = time_index % period;
        steps.get(&offset).unwrap().clone()
    }


    // ========== PRIVATE METHODS (MOVES) =================

    fn exchange(&mut self, args: Vec<String>) {
        let hm = self.dancers.clone();
        let i1 = args[0].parse::<u8>().unwrap();
        let i2 = args[1].parse::<u8>().unwrap();
        let s1 = hm.get(&i1).unwrap().clone();
        let s2 = hm.get(&i2).unwrap().clone();
        self.dancers.insert(i1, s2.clone());
        self.dancers.insert(i2, s1.clone());
        self.positions.insert(s1, i2);
        self.positions.insert(s2, i1);
    }

    fn partner(&mut self, args: Vec<String>) {
        let hm = self.positions.clone();
        let s1 = args[0].clone();
        let s2 = args[1].clone();
        let i1 = hm.get(&s1).unwrap().clone();
        let i2 = hm.get(&s2).unwrap().clone();
        self.dancers.insert(i1, s2.clone());
        self.dancers.insert(i2, s1.clone());
        self.positions.insert(s1, i2);
        self.positions.insert(s2, i1);
    }

    fn spin(&mut self, args: Vec<String>) {
        let hm     = self.dancers.clone();
        let length = args[0].parse::<u8>().unwrap();
        for (k, v) in hm {
            let k1 = (k + length) % self.size;
            self.dancers.insert(k1, v);
        }
        for (k, v) in self.dancers.clone() {
            self.positions.insert(v, k);
        }
    }


    // ========== PRIVATE METHODS (HELPERS) ===============

    fn choreograph(&mut self) -> (HashMap<u64, String>, u64) {
        let mut steps:  HashMap<u64, String> = HashMap::new();
        let mut states: HashMap<String, u64> = HashMap::new();
        let mut count: u64                   = 0;
        states.insert(self.state(), count);
        loop {
            count    += 1;
            let state = self.dance();
            if states.contains_key(&state) {
                break;
            } else {
                states.insert(state, count);
            }
        }
        for (k, v) in states {
            steps.insert(v, k);
        }
        let period = steps.len() as u64;
        (steps, period)
    }

    fn sorted_dancers(&self) -> Vec<String> {
        let mut keys: Vec<u8> = self.dancers.clone().into_keys().collect();
        keys.sort_unstable();
        keys
            .iter()
            .map(|k| self.dancers.get(k).unwrap().clone())
            .collect()
    }

    fn state(&self) -> String {
        String::from_iter(self.sorted_dancers())
    }
}