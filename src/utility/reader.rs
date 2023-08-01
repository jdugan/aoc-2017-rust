use std::fs::read_to_string;

// ----------------------------------------------------
// Public Methods
// ----------------------------------------------------

pub fn to_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn split_word(word: String) -> Vec<String> {
    word.split_terminator("")
        .skip(1)
        .map(String::from)
        .collect()
}