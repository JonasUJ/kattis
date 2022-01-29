use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let mut history = Vec::<i32>::with_capacity(100);
    let mut cur = 0;
    history.push(cur);
    let students: i32 = lines
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let line = lines.next().unwrap();
    let mut cmds = line.split_whitespace();
    while let Some(cmd) = cmds.next() {
        match cmd {
            "undo" => {
                history.truncate(history.len() - cmds.next().unwrap().parse::<usize>().unwrap());
                cur = *history.last().unwrap();
            },
            n => {
                let n: i32 = n.parse().unwrap();
                cur = (students + (cur + n) % students) % students;
                history.push(cur);
            }
        }
    }

    print!("{}", cur);
}
