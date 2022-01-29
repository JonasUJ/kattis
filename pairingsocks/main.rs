use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let pairs: usize = lines.next().unwrap().parse().unwrap();
    let mut stack: Vec<&str> = Vec::with_capacity(pairs);
    let line = lines.next().unwrap();

    for sock in line.split_whitespace() {
        if let Some(other) = stack.last() {
            if sock == *other {
                stack.pop();
                continue;
            }
        }

        stack.push(sock);
    }

    if stack.is_empty() {
        print!("{}", pairs * 2);
    } else {
        print!("impossible");
    }
}
