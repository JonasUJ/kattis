use std::{
    collections::BTreeMap,
    io::{self, BufRead},
    ops::Bound,
};

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Departure(usize);

impl Departure {
    fn parse(input: &str) -> Self {
        let mut dep = Departure(0);
        let mut iter = input.split(':');
        dep.0 += 3600 * iter.next().unwrap().parse::<usize>().unwrap();
        dep.0 += 60 * iter.next().unwrap().parse::<usize>().unwrap();
        dep.0 += iter.next().unwrap().parse::<usize>().unwrap();
        dep
    }

    fn seconds(&self) -> usize {
        self.0 % 60
    }

    fn minutes(&self) -> usize {
        self.0 / 60 % 60
    }

    fn hours(&self) -> usize {
        self.0 / 3600
    }

    fn delay(&mut self, seconds: usize) {
        let sec = (self.seconds() + seconds) % 60;
        let min = (self.minutes() + (seconds + self.seconds()) / 60) % 60;
        let hour = (self.hours() + (seconds + self.seconds() + self.minutes() * 60) / 3600) % 24;
        self.0 = 3600 * hour;
        self.0 += 60 * min;
        self.0 += sec;
    }
}

impl ToString for Departure {
    fn to_string(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}",
            self.hours(),
            self.minutes(),
            self.seconds()
        )
    }
}

impl std::fmt::Debug for Departure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Departure").field(&self.to_string()).finish()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut timetable = BTreeMap::<Departure, String>::new();
    let eod = Bound::Included(Departure::parse("23:59:59"));
    let mut ouput = Vec::<String>::new();

    for line in stdin.lock().lines().skip(1).map(Result::unwrap) {
        let mut iter = line.split_whitespace();
        match iter.next().unwrap() {
            "cancel" => {
                timetable.remove(&Departure::parse(iter.next().unwrap()));
            }
            "delay" => {
                let mut dep = Departure::parse(iter.next().unwrap());
                let dest = timetable.remove(&dep).unwrap();
                dep.delay(iter.next().unwrap().parse().unwrap());
                timetable.insert(dep, dest);
            }
            "reroute" => {
                timetable
                    .entry(Departure::parse(iter.next().unwrap()))
                    .and_modify(|dest| *dest = iter.next().unwrap().to_string());
            }
            "destination" => ouput.push(
                if let Some(dest) = timetable.get(&Departure::parse(iter.next().unwrap())) {
                    dest.to_owned()
                } else {
                    "-".to_string()
                },
            ),
            "next" => {
                let mut range =
                    timetable.range((Bound::Included(Departure::parse(iter.next().unwrap())), eod));
                let next = range.next().unwrap();
                ouput.push(format!("{} {}", next.0.to_string(), next.1))
            }
            "count" => ouput.push(
                timetable
                    .range((
                        Bound::Included(Departure::parse(iter.next().unwrap())),
                        Bound::Included(Departure::parse(iter.next().unwrap())),
                    ))
                    .count()
                    .to_string(),
            ),
            time => {
                timetable.insert(Departure::parse(time), iter.next().unwrap().to_string());
            }
        }
    }

    print!("{}", ouput.join("\n"));
}
