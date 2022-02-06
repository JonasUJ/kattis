use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    A,
    B,
}

impl Color {
    fn invert(&self) -> Self {
        if let &Color::A = self {
            Color::B
        } else {
            Color::A
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vertex {
    next: u32,
    color: Option<Color>,
    invert: bool,
}

impl Vertex {
    fn new(n: u32) -> Self {
        Self {
            next: n,
            color: None,
            invert: false,
        }
    }

    fn color(&self, invert: bool) -> Option<Color> {
        self.color.map(|c| if invert { c.invert() } else { c })
    }
}

#[derive(PartialEq, Eq)]
enum State {
    Harmony,
    Conflict,
}

impl State {
    fn new(c: &str) -> Self {
        if c == "0" {
            Self::Harmony
        } else {
            Self::Conflict
        }
    }

    fn compat(&self, a: Color, b: Color) -> bool {
        a == b && *self == Self::Harmony || a != b && *self == Self::Conflict
    }
}

#[derive(Debug)]
struct UnionFind {
    id: Vec<Vertex>,
    sz: Vec<u32>,
}

impl UnionFind {
    fn new(count: u32) -> Self {
        let mut id = Vec::with_capacity(count as usize);
        id.extend((0..count).map(Vertex::new));
        UnionFind {
            id,
            sz: vec![1; count as usize],
        }
    }

    unsafe fn find_unchecked(&mut self, mut vert: Vertex) -> (Vertex, bool) {
        let mut invert = vert.invert;
        while vert != *self.id.get_unchecked(vert.next as usize) {
            vert = *self.id.get_unchecked(vert.next as usize);
            if vert.invert {
                invert = !invert;
            }
        }

        (vert, invert)
    }

    unsafe fn connect_unchecked(&mut self, a: u32, b: u32, state: State) -> bool {
        let a_vert = *self.id.get_unchecked(a as usize);
        let b_vert = *self.id.get_unchecked(b as usize);
        let (a_root, a_invert) = self.find_unchecked(a_vert);
        let (b_root, b_invert) = self.find_unchecked(b_vert);

        if a_root == b_root {
            state.compat(
                a_vert.color(a_invert).unwrap(),
                b_vert.color(b_invert).unwrap(),
            )
        } else {
            let a_gt_b = *self.sz.get_unchecked(a_root.next as usize)
                > *self.sz.get_unchecked(b_root.next as usize);

            match (a_vert.color(a_invert), state, b_vert.color(b_invert)) {
                (None, State::Harmony, None) => {
                    self.id.get_unchecked_mut(a as usize).color = Some(Color::A);
                    self.id.get_unchecked_mut(b as usize).color = Some(Color::A);
                }
                (None, State::Conflict, None) => {
                    self.id.get_unchecked_mut(a as usize).color = Some(Color::A);
                    self.id.get_unchecked_mut(b as usize).color = Some(Color::B);
                }
                (None, State::Harmony, Some(cb)) => {
                    self.id.get_unchecked_mut(a as usize).color = Some(cb)
                }
                (None, State::Conflict, Some(cb)) => {
                    self.id.get_unchecked_mut(a as usize).color = Some(cb.invert())
                }
                (Some(ca), State::Harmony, None) => {
                    self.id.get_unchecked_mut(b as usize).color = Some(ca)
                }
                (Some(ca), State::Conflict, None) => {
                    self.id.get_unchecked_mut(b as usize).color = Some(ca.invert())
                }
                (Some(ca), State::Harmony, Some(cb)) if a_gt_b => {
                    self.id.get_unchecked_mut(b_root.next as usize).invert = ca != cb
                }
                (Some(ca), State::Conflict, Some(cb)) if a_gt_b => {
                    self.id.get_unchecked_mut(b_root.next as usize).invert = ca == cb
                }
                (Some(ca), State::Harmony, Some(cb)) => {
                    self.id.get_unchecked_mut(a_root.next as usize).invert = ca != cb
                }
                (Some(ca), State::Conflict, Some(cb)) => {
                    self.id.get_unchecked_mut(a_root.next as usize).invert = ca == cb
                }
            }

            if a_gt_b {
                self.id.get_unchecked_mut(b_root.next as usize).next = a_root.next;
                *self.sz.get_unchecked_mut(a_root.next as usize) +=
                    *self.sz.get_unchecked(b_root.next as usize);
            } else {
                self.id.get_unchecked_mut(a_root.next as usize).next = b_root.next;
                *self.sz.get_unchecked_mut(b_root.next as usize) +=
                    *self.sz.get_unchecked(a_root.next as usize);
            }

            true
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);
    let line = lines.next().unwrap();
    let mut line_iter = line.split_whitespace();
    let n: usize = line_iter.next().unwrap().parse().unwrap();
    let _m: usize = line_iter.next().unwrap().parse().unwrap();

    let mut uf = UnionFind::new(n as u32);

    for line in lines {
        let mut line_iter = line.split_whitespace();
        let u: u32 = line_iter.next().unwrap().parse().unwrap();
        let v: u32 = line_iter.next().unwrap().parse().unwrap();
        let c = line_iter.next().unwrap();

        unsafe {
            if !uf.connect_unchecked(u, v, State::new(c)) {
                print!("0");
                return;
            }
        }
    }

    print!("1");
}
