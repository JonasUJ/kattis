use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    lines.next();

    let mut volume = 7;

    for l in lines {
        match l.as_str() {
            "Skru op!" if volume < 10 => volume += 1,
            "Skru ned!" if volume > 0 => volume -= 1,
            _ => continue,
        }
    }

    print!("{}", volume);
}
