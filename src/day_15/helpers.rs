#[derive(Debug)]
pub struct Generator {
    pub seed:     u64,
    pub factor:   u64,
    pub divisor:  u64,
    pub value:    u64
}

impl Generator {
    // ========== CLASS METHODS ===========================

    pub fn new(seed: u64, factor: u64, divisor: u64) -> Generator {
        Generator{ seed, factor, divisor, value: seed.clone() }
    }


    // ========== PUBLIC METHODS ==========================

    pub fn next(&mut self) {
        loop {
            self.value = (self.value * self.factor) % 2147483647;
            if self.value % self.divisor == 0 {
                break;
            }
        }
    }
}