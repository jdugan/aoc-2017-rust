// --------------------------------------------------------
// Particle
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Particle {
    pub id:  u32,
    pub px:  i64,
    pub py:  i64,
    pub pz:  i64,
    pub vx:  i64,
    pub vy:  i64,
    pub vz:  i64,
    pub ax:  i64,
    pub ay:  i64,
    pub az:  i64,
}

impl Particle {
    // ========== PUBLIC METHODS ==========================

    pub fn tick(&mut self) -> (u64, u64) {
        self.vx += self.ax;
        self.vy += self.ay;
        self.vz += self.az;
        self.px += self.vx;
        self.py += self.vy;
        self.pz += self.vz;

        let distance = self.manhattan_distance();
        let speed    = self.combined_speed();

        (distance, speed)
    }

    pub fn combined_speed(&self) -> u64 {
        (self.vx.abs() + self.vy.abs() + self.vz.abs()) as u64
    }

    pub fn manhattan_distance(&self) -> u64 {
        (self.px.abs() + self.py.abs() + self.pz.abs()) as u64
    }
}