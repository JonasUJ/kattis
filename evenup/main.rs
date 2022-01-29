use std::io::{self, BufRead};

#[derive(PartialEq, Eq)]
enum Parity {
    Even,
    Odd,
}

fn parity(s: &str) -> Parity {
    match s.get(s.len() - 1..).unwrap() {
        "0" | "2" | "4" | "6" | "8" => Parity::Even,
        _ => Parity::Odd,
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let cards: usize = lines.next().unwrap().parse().unwrap();
    let mut stack: Vec<Parity> = Vec::with_capacity(cards);
    let line = lines.next().unwrap();

    for card in line.split_whitespace().map(parity) {
        if let Some(other) = stack.last() {
            if card == *other {
                stack.pop();
                continue;
            }
        }

        stack.push(card);
    }

    print!("{}", stack.len());
}
