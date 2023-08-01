use std::fs::read_to_string;

// ----------------------------------------------------
// Public Methods
// ----------------------------------------------------

pub fn to_chars(filename: &str) -> Vec<char> {
    to_lines(filename)[0]
        .chars()
        .collect()
}

pub fn to_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}