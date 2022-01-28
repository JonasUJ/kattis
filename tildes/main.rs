use std::fmt::Write;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Site(usize);

#[derive(Debug)]
struct UnionFind {
    id: Vec<Site>,
    sz: Vec<usize>,
}

impl UnionFind {
    fn new(count: usize) -> Self {
        UnionFind {
            id: (0..count).map(Site).collect(),
            sz: vec![1; count],
        }
    }

    fn find(&mut self, mut site: Site) -> Site {
        while site != self.id[site.0] {
            self.id[site.0] = self.id[self.id[site.0].0];
            site = self.id[site.0];
        }

        site
    }

    fn union(&mut self, a: Site, b: Site) {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return;
        }

        if self.sz[a.0] < self.sz[b.0] {
            self.id[a.0] = b;
            self.sz[b.0] += self.sz[a.0];
        } else {
            self.id[b.0] = a;
            self.sz[a.0] += self.sz[b.0];
        }
    }

    fn size(&mut self, site: Site) -> usize {
        let set = self.find(site);
        self.sz[set.0]
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line);
    let mut uf = UnionFind::new(line.split(' ').next().unwrap().parse().unwrap());
    let mut output = String::with_capacity(line.split(' ').next().unwrap().parse::<usize>().unwrap() * 5);

    for line in stdin.lock().lines().map(Result::unwrap) {
        let mut iter = line.split_whitespace();
        match iter.next() {
            Some("t") => uf.union(
                Site(iter.next().unwrap().parse::<usize>().unwrap() - 1),
                Site(iter.next().unwrap().parse::<usize>().unwrap() - 1),
            ),
            Some("s") => {
                writeln!(
                    &mut output,
                    "{}",
                    uf.size(Site(iter.next().unwrap().parse::<usize>().unwrap() - 1)),
                );
            }
            _ => panic!("invalid command"),
        }
    }

    print!("{}", output);
}
