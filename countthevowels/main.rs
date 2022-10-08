use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let vowels = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .next()
        .unwrap()
        .chars()
        .fold(0, |a, c| {
            if let 'A' | 'E' | 'I' | 'O' | 'U' = c.to_ascii_uppercase() {
                a + 1
            } else {
                a
            }
        });
    print!("{}", vowels);
}
