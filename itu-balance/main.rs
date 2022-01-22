use std::io;

fn main() {
    let stdin = io::stdin();
    let mut line = String::with_capacity(200_000);
    stdin.read_line(&mut line);
    let mut stack = Vec::with_capacity(200_000);

    for c in line.chars() {
        match c {
            '[' | '(' => stack.push(c),
            ']' => {
                if stack.pop() != Some('[') {
                    print!("{}", 0);
                    return;
                }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    print!("{}", 0);
                    return;
                }
            }
            '\n' => break,
            _ => panic!("unexpected character"),
        }
    }

    if stack.is_empty() {
        print!("{}", 1);
    } else {
        print!("{}", 0);
    }
}
