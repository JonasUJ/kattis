use std::fmt::Write;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Site(u32);

#[derive(Debug)]
struct UnionFind {
    id: Vec<Site>,
    sz: Vec<u32>,
}

impl UnionFind {
    fn new(count: u32) -> Self {
        UnionFind {
            id: (0..count).map(Site).collect(),
            sz: vec![1; count as usize],
        }
    }

    unsafe fn find_unchecked(&mut self, mut site: Site) -> Site {
        while site != *self.id.get_unchecked(site.0 as usize) {
            *self.id.get_unchecked_mut(site.0 as usize) = *self
                .id
                .get_unchecked(self.id.get_unchecked(site.0 as usize).0 as usize);
            site = *self.id.get_unchecked(site.0 as usize);
        }

        site
    }

    unsafe fn union_unchecked(&mut self, a: Site, b: Site) {
        let a = self.find_unchecked(a);
        let b = self.find_unchecked(b);

        if a == b {
            return;
        }

        if *self.sz.get_unchecked(a.0 as usize) < *self.sz.get_unchecked(b.0 as usize) {
            *self.id.get_unchecked_mut(a.0 as usize) = b;
            *self.sz.get_unchecked_mut(b.0 as usize) += *self.sz.get_unchecked(a.0 as usize);
        } else {
            *self.id.get_unchecked_mut(b.0 as usize) = a;
            *self.sz.get_unchecked_mut(a.0 as usize) += *self.sz.get_unchecked(b.0 as usize);
        }
    }

    unsafe fn connected(&mut self, a: Site, b: Site) -> bool {
        self.find_unchecked(a) == self.find_unchecked(b)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line);
    let mut uf = UnionFind::new(line.split(' ').next().unwrap().parse().unwrap());
    let mut output =
        String::with_capacity(line.split(' ').next().unwrap().parse::<usize>().unwrap() * 5);

    for line in stdin.lock().lines().map(Result::unwrap) {
        let mut iter = line.split_whitespace();

        // SAFTEY: lol
        unsafe {
            match iter.next() {
                Some("=") => uf.union_unchecked(
                    Site(iter.next().unwrap().parse::<u32>().unwrap()),
                    Site(iter.next().unwrap().parse::<u32>().unwrap()),
                ),
                Some("?") => {
                    writeln!(
                        &mut output,
                        "{}",
                        if uf.connected(
                            Site(iter.next().unwrap().parse::<u32>().unwrap()),
                            Site(iter.next().unwrap().parse::<u32>().unwrap())
                        ) {
                            "yes"
                        } else {
                            "no "
                        },
                    );
                }
                _ => panic!("invalid command"),
            }
        }
    }

    print!("{}", output);
}
