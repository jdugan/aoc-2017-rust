use std::collections::HashMap;

use crate::common::grid::Point;

// --------------------------------------------------------
// Virus
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Virus {
    pub location:        (i32, i32),
    pub direction:       String,
    pub burst_count:     usize,
    pub infection_count: usize,
}

impl Virus {
    // ========== CLASS METHODS ===========================

    pub fn new() -> Virus {
        let location        = (0_i32, 0_i32);
        let direction       = String::from("N");
        let burst_count     = 0;
        let infection_count = 0;

        Virus{ location, direction, burst_count, infection_count }
    }

    // ========== PUBLIC METHODS ==========================

    pub fn evolved_burst(&mut self, mut points: HashMap<(i32, i32), String>) -> HashMap<(i32, i32), String> {
        self.burst_count += 1;
        match points.get(&self.location) {
            Some(s)  => match s.as_str() {
                "F" => {
                    self.turn_right();
                    self.turn_right();
                    points.remove(&self.location);
                }
                "W" => {
                    points.insert(self.location, String::from("#"));
                    self.infection_count += 1;
                },
                "#" => {
                    self.turn_right();
                    points.insert(self.location, String::from("F"));
                },
                _ => {
                    println!("ERROR ::evolved_burst - invalid state {} encountered.", s);
                }
            },
            None => {
                self.turn_left();
                points.insert(self.location, String::from("W"));
            }
        }
        self.move_forward();
        points
    }

    pub fn simple_burst(&mut self, mut points: HashMap<(i32, i32), String>) -> HashMap<(i32, i32), String> {
        self.burst_count += 1;
        match points.get(&self.location) {
            Some(_)  => {
                self.turn_right();
                points.remove(&self.location);
            },
            None => {
                self.turn_left();
                points.insert(self.location, String::from("#"));
                self.infection_count += 1;
            }
        }
        self.move_forward();
        points
    }


    // ========== PRIVATE METHODS =========================

    fn move_forward(&mut self) {
        let point     = Point::new(self.location.0, self.location.1);
        self.location = match self.direction.as_str() {
            "N" => point.north_id(),
            "E" => point.east_id(),
            "S" => point.south_id(),
            "W" => point.west_id(),
            _   => {
                println!("ERROR ::move_forward - invalid direction {} received.", self.direction);
                (0_i32, 0_i32)
            }
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction.as_str() {
            "N" => String::from("W"),
            "W" => String::from("S"),
            "S" => String::from("E"),
            "E" => String::from("N"),
            _   => {
                println!("ERROR ::turn_left - invalid direction {} received.", self.direction);
                String::from("N")
            }
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction.as_str() {
            "N" => String::from("E"),
            "E" => String::from("S"),
            "S" => String::from("W"),
            "W" => String::from("N"),
            _   => {
                println!("ERROR ::turn_right - invalid direction {} received.", self.direction);
                String::from("N")
            }
        }
    }
}