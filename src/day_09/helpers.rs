#[derive(Debug)]
#[derive(Default)]
pub struct Parser {
    pub output:   String,
    pub bin_size: u32,
    pub score:    u32
}

impl Parser {
    // ----------------------------------------------------
    // Public Methods
    // ----------------------------------------------------

    // ========== SCORING =================================

    pub fn calculate_score(&mut self) {
        let mut level = 0_u32;
        let mut score = 0_u32;
        let mut chars = self.output.chars();
        loop {
            match chars.next() {
                Some(c) => {
                    if c == '{' {
                        level += 1;
                        score += level;
                    } else if c == '}' {
                        level -= 1;
                    }
                },
                None    => break
            }
        }
        self.score = score;
    }


    // ========== COMPRESSION =============================

    pub fn remove_annotations(&mut self) {
        let mut output: Vec<char> = vec![];
        let mut chars = self.output.chars();
        loop {
            match chars.next() {
                Some(c) => {
                    if c == '!' {
                        chars.next();
                    } else {
                        output.push(c);
                    }
                },
                None => break
            }
        }
        self.output = output.into_iter().collect::<String>();
    }

    pub fn remove_garbage(&mut self) {
        let mut output: Vec<char> = vec![];
        let mut chars   = self.output.chars();
        let mut pending = false;
        let mut count   = 0_u32;
        loop {
            match chars.next() {
                Some(c) => {
                    if pending {
                        if c == '>' {
                            pending = false;
                        } else {
                            count += 1;
                        }
                    } else {
                        if c == '<' {
                            pending = true;
                        } else {
                            output.push(c);
                        }
                    }
                },
                None => break
            }
        }
        self.output   = output.into_iter().collect::<String>();
        self.bin_size = count;
    }

    pub fn remove_punctuation(&mut self) {
        let mut s0: String = "".to_string();
        let mut s1: String = self.output.clone();
        while s0 != s1 {
            s0 = s1.clone();
            s1 = s0.replace(",,", "");
        }
        s0 = s0.replace("{,}", "{}");
        s0 = s0.replace("{,{", "{{");
        s0 = s0.replace("},}", "}}");
        self.output = s0;
    }
}