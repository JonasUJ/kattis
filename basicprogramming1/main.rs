use std::{borrow::Cow, cmp::Ordering, io};

fn main() {
    let stdin = io::stdin();

    let mut line = String::new();
    stdin.read_line(&mut line);
    let mut iter = line.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let t = iter.next().unwrap();

    let mut line = String::new();
    stdin.read_line(&mut line);
    let mut a = line.split_whitespace().map(|n| n.parse::<usize>().unwrap());

    let output = match t {
        "1" => Cow::Borrowed("7"),
        "2" => Cow::Borrowed(match a.next().unwrap().cmp(&a.next().unwrap()) {
            Ordering::Greater => "Bigger",
            Ordering::Equal => "Equal",
            Ordering::Less => "Smaller",
        }),
        "3" => Cow::Owned(
            a.clone()
                .take(3)
                .find(|n| {
                    n != &a.clone().take(3).min().unwrap() && n != &a.clone().take(3).max().unwrap()
                })
                .unwrap()
                .to_string(),
        ),
        "4" => Cow::Owned(a.sum::<usize>().to_string()),
        "5" => Cow::Owned(a.filter(|n| *n & 1 == 0).sum::<usize>().to_string()),
        "6" => Cow::Owned(
            a.map(|n| (n % 26 + 97) as u32)
                .map(std::char::from_u32)
                .map(Option::unwrap)
                .collect::<String>(),
        ),
        "7" => {
            let mut a_vec = Vec::<usize>::with_capacity(n);
            a_vec.extend(a);
            let mut i = 0;
            let mut visited = vec![false; n];

            Cow::Borrowed(loop {
                if visited[i] {
                    break "Cyclic";
                } else {
                    visited[i] = true;
                }

                i = a_vec[i] as usize;

                if i >= n {
                    break "Out";
                } else if i == n - 1 {
                    break "Done";
                }
            })
        }
        _ => panic!("invalid command"),
    };

    print!("{}", output);
}
