use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Grade {
    A,
    B,
    C,
    D,
    E,
    FX,
    F,
}

#[derive(Eq, PartialEq)]
struct Student {
    name: String,
    grade: Grade,
    grain: i32,
}

impl Student {
    fn new(name: String, grade: Grade, grain: i32) -> Self {
        Student { name, grade, grain }
    }
}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> Ordering {
        self.grade
            .cmp(&other.grade)
            .then_with(|| self.grain.cmp(&other.grain))
            .then_with(|| self.name.cmp(&other.name))
    }
}

// Kattis is outdated
fn split_once(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ' ');
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

fn main() {
    let stdin = io::stdin();
    let mut students = stdin
        .lock()
        .lines()
        .skip(1)
        .map(Result::unwrap)
        .map(|line| {
            let (name, grades) = split_once(&line);
            let mut chars = grades.chars();
            let mut grain = 0i32;
            let grade = match chars.next().expect("expected grade letter") {
                'A' => Grade::A,
                'B' => Grade::B,
                'C' => Grade::C,
                'D' => Grade::D,
                'E' => Grade::E,
                'F' => {
                    if let Some('X') = chars.next() {
                        Grade::FX
                    } else {
                        Grade::F
                    }
                }
                _ => panic!("unknown grade"),
            };

            let mut chars = grades.chars().rev();
            loop {
                match chars.next() {
                    Some('-') => grain += 1,
                    Some('+') => grain -= 1,
                    _ => break,
                }
            }

            Student::new(name.to_string(), grade, grain)
        })
        .collect::<Vec<Student>>();

    students.sort();
    print!(
        "{}",
        students
            .into_iter()
            .map(|student| student.name)
            .collect::<Vec<_>>()
            .join("\n")
    );
}
