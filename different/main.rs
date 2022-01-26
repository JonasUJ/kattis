use std::io::{self, BufRead};

fn abs_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn main() {
    print!(
        "{}",
        io::stdin()
            .lock()
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                let mut iter = line.split_whitespace();
                abs_diff(
                    iter.next().unwrap().parse::<usize>().unwrap(),
                    iter.next().unwrap().parse::<usize>().unwrap(),
                )
                .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
}
