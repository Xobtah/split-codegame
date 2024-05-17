# split-codegame
I got this exercise as a live coding session for a technical interview and I thought it was an interesting exercise.

The solution I wrote during the interview is in `src/main.rs`.

## Exercise
```Rust
struct Split {
    // TODO
}

impl Split {
    fn new(input: &str, delimiter: char) -> Self {
        // TODO
    }
}

impl Iterator for Split {
    type Item = &str;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO
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
```
