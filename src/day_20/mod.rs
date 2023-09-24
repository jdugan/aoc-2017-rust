pub mod helpers;

use std::collections::HashSet;

use crate::day_20::helpers::Particle;
use crate::utility::reader;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn day() -> u8 {
    20
}

pub fn puzzle1() -> u32 {
    let mut particles = data();
    let mut ticks     = 0;
    let     max_ticks = 350;
    let mut closest   = get_closest(&particles);

    while ticks < max_ticks {
        particles = particles.into_iter().map(|mut p| { p.tick(); p }).collect();
        closest   = get_closest(&particles);
        ticks    += 1;
    }
    closest.id
}

pub fn puzzle2() -> usize {
    let mut particles = data();
    let mut ticks     = 0;
    let     max_ticks = 100;

    while ticks < max_ticks {
        particles = particles.into_iter().map(|mut p| { p.tick(); p }).collect();
        particles = remove_collisions(&particles);
        ticks    += 1;
    }
    particles.len()
}


// --------------------------------------------------------
// Private Methods
// --------------------------------------------------------

// ========== HELPERS =====================================

fn get_closest(particles: &Vec<Particle>) -> Particle {
    let mut min_ref = particles[0].clone();
    let mut min_val = u64::MAX;
    for p in particles {
        let current = p.manhattan_distance();
        if current < min_val {
            min_ref = p.clone();
            min_val = current;
        }
    }
    min_ref
}

fn remove_collisions(particles: &Vec<Particle>) -> Vec<Particle> {
    let mut positions:  HashSet<(i64, i64, i64)> = HashSet::new();
    let mut collisions: HashSet<(i64, i64, i64)> = HashSet::new();
    for p in particles {
        let key  = (p.px, p.py, p.pz);
        let open = positions.insert(key);
        if !open {
            collisions.insert(key);
        }
    }
    particles.iter().fold(vec![], |mut v, p| {
        let key  = (p.px, p.py, p.pz);
        if !collisions.contains(&key) {
            v.push(p.clone());
        }
        v
    })
}


// ========== DATA ========================================

fn data() -> Vec<Particle> {
    reader::to_strings("./data/day20/input.txt")
        .iter()
        .enumerate()
        .map(|(id, line)| parse_line(id, line))
        .collect()
}

fn parse_line(id: usize, line: &String) -> Particle {
    let mut list = line.clone();
    list = list.replace("p=<", "");
    list = list.replace(" v=<", "");
    list = list.replace(" a=<", "");
    list = list.replace(">", "");
    let mut parts = list.split(",");
    Particle {
        id: id as u32,
        px: parts.next().unwrap().parse::<i64>().unwrap(),
        py: parts.next().unwrap().parse::<i64>().unwrap(),
        pz: parts.next().unwrap().parse::<i64>().unwrap(),
        vx: parts.next().unwrap().parse::<i64>().unwrap(),
        vy: parts.next().unwrap().parse::<i64>().unwrap(),
        vz: parts.next().unwrap().parse::<i64>().unwrap(),
        ax: parts.next().unwrap().parse::<i64>().unwrap(),
        ay: parts.next().unwrap().parse::<i64>().unwrap(),
        az: parts.next().unwrap().parse::<i64>().unwrap(),
    }
}


// --------------------------------------------------------
// Unit Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_20() {
        assert_eq!(puzzle1(), 300);
        assert_eq!(puzzle2(), 502);
    }
}