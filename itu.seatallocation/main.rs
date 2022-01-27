use std::{
    collections::BinaryHeap,
    io::{self, BufRead},
};

#[derive(Eq, Ord)]
struct Party<'a> {
    seats: &'a mut usize,
    votes: usize,
    quotient: usize,
}

impl<'a> PartialEq for Party<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.seats == other.seats
    }
}

impl<'a> PartialOrd for Party<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.quotient.partial_cmp(&other.quotient) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.votes.partial_cmp(&other.votes)
    }
}

impl<'a> Party<'a> {
    fn assign_seat(&mut self) {
        *self.seats += 1;
        self.quotient = self.votes / (*self.seats + 1);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let line = lines.next().unwrap();
    let line: Vec<_> = line.split_whitespace().collect();
    let num_parties: usize = line[0].parse().unwrap();
    let num_seats: usize = line[1].parse().unwrap();

    let mut seats = vec![0; num_parties];
    let mut seats_iter = seats.iter_mut();
    let mut heap = BinaryHeap::with_capacity(num_seats);
    for votes in lines.map(|line| line.parse::<usize>().unwrap()) {
        heap.push(Party {
            seats: seats_iter.next().unwrap(),
            votes,
            quotient: votes,
        })
    }

    (0..num_seats).for_each(|_| heap.peek_mut().unwrap().assign_seat());

    print!(
        "{}",
        seats
            .iter()
            .take(num_parties)
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
