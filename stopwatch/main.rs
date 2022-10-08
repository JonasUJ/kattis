use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let n: i32 = lines.next().unwrap().parse().unwrap();

    if n & 1 == 1 {
        print!("still running");
        return;
    }

    let secs: i32 = lines
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .chunks(2)
        .fold(0, |a, t| a + t[1] - t[0]);

    print!("{}", secs);
}
