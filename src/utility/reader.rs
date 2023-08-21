use std::fs::read_to_string;


// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

// ========== NUMBERS =====================================

pub fn to_signed_ints(filename: &str) -> Vec<i32> {
    read_to_string(filename)
    .unwrap()
    .lines()
    .map(|line| line.parse::<i32>().unwrap())
    .collect()
}

pub fn to_unsigned_ints(filename: &str) -> Vec<u32> {
    read_to_string(filename)
    .unwrap()
    .lines()
    .map(|line| line.parse::<u32>().unwrap())
    .collect()
}


// ========== STRINGS =====================================

pub fn to_strings(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn to_word(filename: &str) -> String {
    to_strings(filename)
        .pop()
        .unwrap()
}