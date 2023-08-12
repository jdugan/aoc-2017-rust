use std::collections::HashMap;
use std::fmt;
use std::iter::zip;


pub struct List {
    items:  HashMap<u32, u32>,
    pub length: u32
}

impl List {
    pub fn new(length: u32) -> List {
        let mut items: HashMap<u32, u32> = HashMap::new();
        for n in 0..length {
            items.insert(n, n);
        }
        List{ items, length }
    }

    pub fn checksum(&self) -> u32 {
        let v0 = self.items.get(&0).unwrap();
        let v1 = self.items.get(&1).unwrap();
        v0 * v1
    }

    pub fn hash(&mut self, length: u32, pos: u32) {
        if length > 1 {
            let mut keys: Vec<u32> = vec![];
            let mut vals: Vec<u32> = vec![];
            for n in pos..pos+length {
                let i = n % self.length;
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

    pub fn sorted_values(&self) -> Vec<u32> {
        let mut keys: Vec<u32>  = self.items.clone().into_keys().collect();
        let mut vals: Vec<u32> = vec![];
        keys.sort_unstable();
        for k in keys {
            vals.push(self.items.get(&k).unwrap().to_owned());
        }
        vals
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.sorted_values())
    }
}