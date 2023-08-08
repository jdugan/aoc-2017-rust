#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    // ----------------------------------------------------
    // Class Methods
    // ----------------------------------------------------

    pub fn new(x: i32, y: i32) -> Point {
        Point{ x: x, y: y }
    }

    pub fn origin() -> Point {
        Point{ x: 0, y: 0 }
    }


    // ----------------------------------------------------
    // Public Methods
    // ----------------------------------------------------

    // ========== IDS =====================================

    //  self
    pub fn id(&self) -> (i32, i32) {
        (self.x, self. y)
    }

    // directions
    pub fn east_id(&self) -> (i32, i32) {
        (self.x + 1, self.y)
    }
    pub fn north_id(&self) -> (i32, i32) {
        (self.x, self.y + 1)
    }
    pub fn northeast_id(&self) -> (i32, i32) {
        (self.x + 1, self.y + 1)
    }
    pub fn northwest_id(&self) -> (i32, i32) {
        (self.x - 1, self.y + 1)
    }
    pub fn south_id(&self) -> (i32, i32) {
        (self.x, self.y - 1)
    }
    pub fn southeast_id(&self) -> (i32, i32) {
        (self.x + 1, self.y - 1)
    }
    pub fn southwest_id(&self) -> (i32, i32) {
        (self.x - 1, self.y - 1)
    }
    pub fn west_id(&self) -> (i32, i32) {
        (self.x - 1, self.y)
    }

    // collections
    pub fn adjacent_ids(&self) -> Vec<(i32, i32)> {
        vec![
            self.east_id(),
            self.north_id(),
            self.northeast_id(),
            self.northwest_id(),
            self.south_id(),
            self.southeast_id(),
            self.southwest_id(),
            self.west_id()
        ]
    }

    // ========== DISTANCE ================================

    pub fn manhatten_distance(&self) -> u32 {
        let dx = self.x.abs() as u32;
        let dy = self.y.abs() as u32;
        dx + dy
    }
}