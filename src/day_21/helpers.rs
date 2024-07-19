use std::collections::HashMap;

use crate::utility::converter;

// --------------------------------------------------------
// Rule
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Rule {
    pub key: String
}

impl Rule {
    // ========== PUBLIC METHODS ==========================

    pub fn permutated_keys(&self) -> Vec<String> {
        // original
        let b0 = Tile::from(&self.key);
        let b1 = b0.rotate();
        let b2 = b1.rotate();
        let b3 = b2.rotate();
        // flip x
        let x0 = b0.flip_x();
        let x1 = x0.rotate();
        let x2 = x1.rotate();
        let x3 = x2.rotate();
        // flip y
        let y0 = b0.flip_y();
        let y1 = y0.rotate();
        let y2 = y1.rotate();
        let y3 = y2.rotate();
        // flip xy
        let xy0 = y0.flip_x();
        let xy1 = xy0.rotate();
        let xy2 = xy1.rotate();
        let xy3 = xy2.rotate();
        // combined keys
        let mut keys: Vec<String> = vec![
            b0.to_key(),
            b1.to_key(),
            b2.to_key(),
            b3.to_key(),
            x0.to_key(),
            x1.to_key(),
            x2.to_key(),
            x3.to_key(),
            y0.to_key(),
            y1.to_key(),
            y2.to_key(),
            y3.to_key(),
            xy0.to_key(),
            xy1.to_key(),
            xy2.to_key(),
            xy3.to_key(),
        ];
        keys.sort_unstable();
        keys.dedup();
        keys
    }
}


// --------------------------------------------------------
// Tile
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Tile {
    pub pixels: Vec<Vec<String>>,
    pub size:   usize
}

impl Tile {
    // ========== CLASS METHODS ===========================

    pub fn from(key: &String) -> Tile {
        let pixels: Vec<Vec<String>> = key.split("/").into_iter().map(|r| {
            converter::string_to_letters(&r.to_string())
        }).collect();
        let size = pixels.len();

        Tile{ pixels, size }
    }

    pub fn of_size(size: usize) -> Tile {
        let mut pixels: Vec<Vec<String>> = vec![];
        for _ in 0..size {
            let mut row: Vec<String> = vec![];
            for _ in 0..size {
                row.push(String::from("."));
            }
            pixels.push(row);
        }
        Tile{ pixels: pixels, size: size }
    }


    // ========== PUBLIC METHODS ==========================

    pub fn count(&self) -> u32 {
        self.pixels.iter()
                   .flatten()
                   .map(|s| String::from(s))
                   .filter(|s| s == "#")
                   .collect::<String>()
                   .len() as u32
    }

    pub fn divide(&self) -> (Vec<Tile>, usize) {
        let divisor = match self.size % 2 == 0 {
            true  => 2,
            false => 3
        };
        let     grid_size            = self.pixels.len()/ divisor;
        let mut new_tiles: Vec<Tile> = vec![];
        for ty in 0..grid_size {
            for tx in 0..grid_size {
                let mut key = String::from("");
                for dy in 0..divisor {
                    for dx in 0..divisor {
                        let x = (tx * divisor) + dx;
                        let y = (ty * divisor) + dy;
                        key = key + &self.pixels[y][x];
                    }
                    if dy != divisor - 1 {
                        key = key + "/";
                    }
                }
                new_tiles.push(Tile::from(&key.to_string()));
            }
        }
        (new_tiles, grid_size)
    }

    pub fn expand(&self, rules: &HashMap<String, String>) -> Tile {
        let (tiles, grid_size) = self.divide();
        let new_tiles = tiles.iter().fold(vec![], |mut v, t| {
            let key  = rules.get(&t.to_key()).unwrap();
            let tile = Tile::from(&key);
            v.push(tile);
            v
        });
        let tile_size = new_tiles[0].size;

        let mut pixels = Tile::of_size(grid_size * tile_size).pixels;
        for (i, tile) in new_tiles.iter().enumerate() {
            let gy = i / grid_size;
            let gx = i % grid_size;
            for (py, row) in tile.pixels.iter().enumerate() {
                for (px, s) in row.iter().enumerate() {
                    let x = (gx * tile_size) + px;
                    let y = (gy * tile_size) + py;
                    pixels[y][x] = s.to_string();
                }
            }
        }
        Tile{ pixels: pixels, size: grid_size * tile_size }
    }

    pub fn flip_x(&self) -> Tile {
        let mut pixels = self.pixels.clone();
        pixels.reverse();
        Tile{ pixels, size: self.size }
    }

    pub fn flip_y(&self) -> Tile {
        let pixels = self.pixels.iter().map(|r| {
            let mut row = r.clone();
            row.reverse();
            row
        }).collect();
        Tile{ pixels, size: self.size }
    }

    pub fn rotate(&self) -> Tile {
        let g          = self.flip_x();
        let mut pixels = vec![];
        for x in 0..self.size {
            let mut row = vec![];
            for y in 0..self.size {
                row.push(g.pixels[y][x].clone());
            }
            pixels.push(row);
        }
        Tile{ pixels, size: self.size }
    }

    pub fn to_key(&self) -> String {
        let mut k = self.pixels.iter().fold("".to_string(), |s, row|
            s + row.join("").as_str() + "/"
        );
        k.pop();
        k
    }
}