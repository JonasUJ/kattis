use std::io::{self, BufRead};

fn main() {
    print!(
        "{}",
        io::stdin()
            .lock()
            .lines()
            .skip(1)
            .map(Result::unwrap)
            .enumerate()
            .filter_map(|(i, e)| if i & 1 == 0 { Some(e) } else { None })
            .collect::<Vec<_>>()
            .join("\n")
    );
}
