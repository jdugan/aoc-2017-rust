// --------------------------------------------------------
// Pipe
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Pipe {
    pub id:    u32,
    pub left:  u32,
    pub right: u32,
}

impl Pipe {
    // ========== PUBLIC METHODS ==========================

    pub fn strength(&self) -> u32 {
        self.left + self.right
    }
}


// --------------------------------------------------------
// Scenario
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Scenario {
    pub pipes:      Vec<Pipe>,
    pub bridge:     Vec<Pipe>,
    pub connector:  u32,
}

impl Scenario {
    // ========== PUBLIC METHODS ==========================

    pub fn add_pipe(&self, pipe: Pipe) -> Scenario {
        let mut scenario = self.clone();
        scenario.pipes = self.pipes.iter().filter(|p| p.id != pipe.id).cloned().collect::<Vec<Pipe>>();
        scenario.bridge.push(pipe.clone());
        scenario.connector = match self.connector == pipe.left {
            true  => pipe.right,
            false => pipe.left
        };
        scenario
    }

    pub fn candidate_pipes(&self) -> Vec<Pipe> {
        self.pipes.iter()
            .filter(|p| {
                p.left == self.connector || p.right == self.connector
            })
            .cloned()
            .collect::<Vec<Pipe>>()
    }

    pub fn strength(&self) -> u32 {
        self.bridge.iter().fold(0_u32, |mut sum, pipe| {
            sum += pipe.strength();
            sum
        })
    }
}