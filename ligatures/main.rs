use std::{collections::{HashMap, HashSet}, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input);
    let nums: Vec<usize> = input.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect();
    let N = nums[0];
    let Q = nums[1];
    let K = nums[2];

    input.clear();
    input.reserve(N - input.capacity());
    stdin.read_line(&mut input);

    let mut ligatures = HashMap::<String, Vec<usize>>::new();
    let mut counts = Vec::<usize>::new();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        for lig in line.as_bytes().chunks(2).map(|b| String::from_utf8_lossy(b).to_string()) {
            if let Some(v) = ligatures.get_mut(&lig) {
                v.push(counts.len());
            } else {
                ligatures.insert(lig, vec!(counts.len()));
            }
        }
        counts.push(0);
    }

    let mut disabled = HashSet::<usize>::new();
    let mut now_disabled = HashSet::<usize>::new();
    for i in 0..input.len()-2 {
        std::mem::swap(&mut disabled, &mut now_disabled);
        disabled.clear();

        if let Some(v) = ligatures.get(&input[i..i+2]) {
            for lig in v {
                if !now_disabled.contains(&lig) {
                    counts[*lig] += 1;
                    disabled.insert(*lig);
                }
            }
        }
    }

    for c in counts {
        println!("{}", c);
    }
}
