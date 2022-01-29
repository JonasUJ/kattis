use std::collections::HashMap;
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
        let mut id = Vec::with_capacity(count as usize);
        id.extend((0..count).map(Site));
        UnionFind {
            id,
            sz: vec![1; count as usize],
        }
    }

    unsafe fn find_unchecked(&mut self, mut site: Site) -> Site {
        if site.0 == self.id.len() as u32 {
            self.id.push(site);
            self.sz.push(1);
        } else {
            while site != *self.id.get_unchecked(site.0 as usize) {
                *self.id.get_unchecked_mut(site.0 as usize) = *self
                    .id
                    .get_unchecked(self.id.get_unchecked(site.0 as usize).0 as usize);
                site = *self.id.get_unchecked(site.0 as usize);
            }
        }

        site
    }

    unsafe fn union_unchecked(&mut self, a: Site, b: Site) -> u32 {
        let a = self.find_unchecked(a);
        let b = self.find_unchecked(b);

        if a == b {
            return *self.sz.get_unchecked(a.0 as usize);
        }

        if *self.sz.get_unchecked(a.0 as usize) < *self.sz.get_unchecked(b.0 as usize) {
            self.union_impl(b, a)
        } else {
            self.union_impl(a, b)
        }
    }

    unsafe fn union_impl(&mut self, bigger: Site, smaller: Site) -> u32 {
        *self.id.get_unchecked_mut(smaller.0 as usize) = bigger;
        let smaller_sz = *self.sz.get_unchecked(smaller.0 as usize);
        let bigger_sz = self.sz.get_unchecked_mut(bigger.0 as usize);
        *bigger_sz += smaller_sz;
        *bigger_sz
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let mut output = String::new();

    for _ in 0..lines.next().unwrap().parse().unwrap() {
        let bonds = lines.next().unwrap().parse().unwrap();
        let mut uf = UnionFind::new(bonds);
        let mut map = HashMap::<String, Site>::new();

        // SAFTEY: ha
        unsafe {
            lines
                .by_ref()
                .take(bonds as usize)
                .map(|line| {
                    let mut iter = line.split_whitespace().map(|name| {
                        let id = map.len() as u32;

                        *map.entry(name.to_owned()).or_insert(Site(id))
                    });

                    uf.union_unchecked(
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                )})
                .for_each(|n| {
                    writeln!(&mut output, "{}", n);
                });
        }
    }

    print!("{}", output);
}
