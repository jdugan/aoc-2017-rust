use std::fs::read_to_string;

// ----------------------------------------------------
// Public Methods
// ----------------------------------------------------

pub fn read_chars(filename: &str) -> Vec<char> {
    read_lines(filename)[0]
        .chars()
        .collect()
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}