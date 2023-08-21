// --------------------------------------------------------
// Public Methods
// --------------------------------------------------------

pub fn string_to_asciis(input: &String) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];
    for c in input.chars() {
        v.push(c as u32);
    }
    v
}

pub fn string_to_ints(input: &String) -> Vec<u32> {
    input
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

pub fn string_to_letters(word: &String) -> Vec<String> {
    word.split_terminator("")
        .skip(1)
        .map(String::from)
        .collect()
}