use crate::utility::converter;

// --------------------------------------------------------
// Image
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Image {
    pub key: String
}

impl Image {
    // ========== PUBLIC METHODS ==========================

    pub fn permutated_keys(&self) -> Vec<String> {
        // original
        let b0 = Grid::from(&self.key);
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
// Pixel Grid
// --------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Grid {
    pub pixels: Vec<Vec<String>>,
    pub size:   usize
}

impl Grid {
    // ========== CLASS METHODS ===========================

    pub fn from(key: &String) -> Grid {
        let pixels: Vec<Vec<String>> = key.split("/").into_iter().map(|r| {
            converter::string_to_letters(&r.to_string())
        }).collect();
        let size = pixels.len();

        Grid{ pixels, size }
    }


    // ========== PUBLIC METHODS ==========================

    pub fn flip_x(&self) -> Grid {
        let mut pixels = self.pixels.clone();
        pixels.reverse();
        Grid{ pixels, size: self.size }
    }

    pub fn flip_y(&self) -> Grid {
        let pixels = self.pixels.iter().map(|r| {
            let mut row = r.clone();
            row.reverse();
            row
        }).collect();
        Grid{ pixels, size: self.size }
    }

    pub fn rotate(&self) -> Grid {
        let g          = self.flip_x();
        let mut pixels = vec![];
        for x in 0..self.size {
            let mut row = vec![];
            for y in 0..self.size {
                row.push(g.pixels[y][x].clone());
            }
            pixels.push(row);
        }
        Grid{ pixels, size: self.size }
    }

    pub fn to_key(&self) -> String {
        let mut k = self.pixels.iter().fold("".to_string(), |s, row|
            s + row.join("").as_str() + "/"
        );
        k.pop();
        k
    }
}