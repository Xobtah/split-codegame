struct Split {
    input: String,
    delimiter: char,
    index: usize,
}

impl Split {
    fn new(input: &str, delimiter: char) -> Self {
        Self {
            input: input.to_string(),
            delimiter,
            index: 0,
        }
    }
}

impl Iterator for Split {
    type Item = String;

    // This solution is not efficient, but it is simple and works.
    // It is the solution I delivered during the interview.
    // After discussing the problem with the interviewer, we said that
    // using String::find would be a better solution.
    fn next(&mut self) -> Option<Self::Item> {
        let mut part = String::new();
        if self.index >= self.input.len() {
            return None;
        }
        for c in self.input[self.index..].chars() {
            self.index += c.len_utf8();
            if c == self.delimiter {
                break;
            }
            part.push(c);
        }
        Some(part)
    }
}

fn main() {
    Split::new("aøbøc", 'ø').for_each(|part| println!("-> {part}"));
    /*
     * Displays:
     * -> a
     * -> b
     * -> c
     */
    Split::new("aøøbøc", 'ø').for_each(|part| println!("-> {part}"));
    /*
     * Displays:
     * -> a
     * -> 
     * -> b
     * -> c
     */
}
