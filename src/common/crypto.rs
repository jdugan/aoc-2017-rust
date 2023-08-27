use std::collections::HashMap;
use std::fmt;
use std::iter::zip;

use crate::utility::converter;


// --------------------------------------------------------
// Knot Hash
// --------------------------------------------------------

#[derive(Debug)]
pub struct KnotHash {
    pub input:  String,
    pub items:  HashMap<u32, u32>
}

impl KnotHash {
    //========== CLASS METHODS ============================

    pub fn new(input: String) -> KnotHash {
        let items = (0_u32..256_u32).fold(HashMap::new(), |mut hm, n| {
            hm.insert(n, n);
            hm
        });
        KnotHash{ input, items }
    }


    // ========== PUBLIC METHODS ==========================

    pub fn checksum(&self) -> u32 {
        let v0 = self.items.get(&0).unwrap();
        let v1 = self.items.get(&1).unwrap();
        v0 * v1
    }

    pub fn shuffle(&mut self) {
        let mut lengths = converter::string_to_asciis(&self.input);
        lengths.extend(vec![17_u32, 31, 73, 47, 23]);
        let rounds  = 64_u32;
        self.perform_shuffle(&lengths, &rounds)
    }

    pub fn quick_shuffle(&mut self) {
        let lengths = converter::string_to_ints(&self.input);
        let rounds  = 1_u32;
        self.perform_shuffle(&lengths, &rounds)
    }

    pub fn to_binary(&self) -> String {
        let letters = converter::string_to_letters(&self.to_hex());
        letters.iter().fold("".to_string(), |mut str, s| {
            let res = u32::from_str_radix(s, 16);
            if res.is_ok() {
                str = format!("{}{:04b}", str, res.unwrap());
            }
            str
        })
    }

    pub fn to_hex(&self) -> String {
        let     values = self.sorted_values();
        let     chunks = values.chunks(16);
        let mut hex    = "".to_string();
        for c in chunks {
            let i = c.iter().fold(0, |a,b| a ^ b);
            hex   = format!("{}{:0>2x}", hex, i);
        }
        hex
    }


    // ========== PRIVATE METHODS =========================

    fn hash(&mut self, shuffle_len: &u32, shuffle_pos: &u32) {
        if shuffle_len > &1 {
            let num_items = self.items.len() as u32;
            let pos       = shuffle_pos.clone();
            let mut keys  = vec![];
            let mut vals  = vec![];
            for n in pos..pos+shuffle_len {
                let i = n % num_items;
                keys.push(i);
                vals.push(self.items.get(&i).unwrap().to_owned())
            }
            vals.reverse();
            let pairs = zip(keys, vals);
            for (k, v) in pairs {
                self.items.insert(k, v);
            }
        }
    }

    fn perform_shuffle(&mut self, lengths: &Vec<u32>, rounds: &u32) {
        let     num_items = self.items.len() as u32;
        let mut skip      = 0_u32;
        let mut pos       = 0_u32;
        for _ in 0..*rounds {
            for slen in lengths {
                self.hash(slen, &pos);
                pos   = (pos + slen + skip) % num_items;
                skip += 1;
            }
        }
    }

    fn sorted_values(&self) -> Vec<u32> {
        let mut keys: Vec<u32>  = self.items.clone().into_keys().collect();
        keys.sort_unstable();
        keys.iter()
            .map(|k| self.items.get(k).unwrap().clone())
            .collect()
    }
}

impl fmt::Display for KnotHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.sorted_values())
    }
}