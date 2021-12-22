use std::{
    collections::{hash_map::Entry, BTreeSet, HashMap},
    io::{self, BufRead},
    rc::Rc,
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Scope(usize);
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Ident(usize);
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Type(Rc<String>);

enum Stmt {
    Enter(Scope),
    Leave,
    Declare(Ident, Type),
    TypeOf(Ident),
}

impl Stmt {
    fn parse(lineno: usize, line: &str) -> Self {
        let mut parts = line.split(' ');
        match parts.next().expect("expected statement") {
            "{" => Stmt::Enter(Scope(lineno)),
            "}" => Stmt::Leave,
            "DECLARE" => Stmt::Declare(
                Ident(hash(parts.next().expect("expected identifier"))),
                Type(Rc::new(parts.next().expect("expected type").to_string())),
            ),
            "TYPEOF" => Stmt::TypeOf(Ident(hash(parts.next().expect("expected identifier")))),
            _ => panic!("invalid statement"),
        }
    }
}

#[derive(Debug)]
struct Decl {
    declared_type: Type,
    scope: Scope,
    shadow: Option<Rc<Decl>>,
}

trait RcDecl {
    fn in_scope(&self, scopes: &BTreeSet<Scope>) -> Option<&Rc<Decl>>;
}

impl RcDecl for Rc<Decl> {
    fn in_scope(&self, scopes: &BTreeSet<Scope>) -> Option<&Rc<Decl>> {
        if !scopes.contains(&self.scope) {
            if let Some(shadow) = &self.shadow {
                shadow.in_scope(scopes)
            } else {
                None
            }
        } else {
            Some(self)
        }
    }
}

enum Output {
    Empty,
    UnDecl,
    MulDecl,
    Type(Type),
}

impl Output {
    fn is_empty(&self) -> bool {
        if let &Output::Empty = self {
            true
        } else {
            false
        }
    }
}

impl Output {
    fn as_str(&self) -> &'static str {
        match self {
            Output::Empty => "",
            Output::UnDecl => "UNDECLARED",
            Output::MulDecl => "MULTIPLE DECLARATION",
            // >_>
            Output::Type(declared_type) => Box::leak(Box::new(declared_type.0.to_owned())),
        }
    }
}

struct Quickscope {
    scopes: BTreeSet<Scope>,
    decls: HashMap<Ident, Rc<Decl>>,
}

impl Quickscope {
    fn new() -> Self {
        let mut scopes = BTreeSet::<Scope>::new();
        scopes.insert(Scope(0));
        Quickscope {
            scopes,
            decls: HashMap::<Ident, Rc<Decl>>::new(),
        }
    }

    fn eval(&mut self, stmts: impl Iterator<Item = Stmt>) -> String {
        let evaluated = stmts.into_iter().map(|stmt| match stmt {
            Stmt::Enter(scope) => self.enter(scope),
            Stmt::Leave => self.leave(),
            Stmt::Declare(ident, declared_type) => self.declare(ident, declared_type),
            Stmt::TypeOf(ident) => self.get_typeof(ident),
        });

        let mut res: Vec<&str> = vec![];
        for output in evaluated.filter(|o| !o.is_empty()) {
            res.push(output.as_str());
            if let Output::MulDecl = output {
                break;
            }
        }
        res.join("\n")
    }

    fn enter(&mut self, scope: Scope) -> Output {
        self.scopes.insert(scope);
        Output::Empty
    }

    fn leave(&mut self) -> Output {
        self.scopes
            .iter()
            .next_back()
            .expect("scope ended unexpectedly");
        Output::Empty
    }

    fn declare(&mut self, ident: Ident, declared_type: Type) -> Output {
        let entry = self.decls.entry(ident);

        if let Entry::Occupied(ref entry) = entry {
            if entry.get().scope
                == *self
                    .scopes
                    .iter()
                    .next_back()
                    .expect("unexpected end of scope")
            {
                return Output::MulDecl;
            }
        }

        entry
            .and_modify(|decl| {
                *decl = Rc::new(Decl {
                    declared_type: declared_type.clone(),
                    scope: *self
                        .scopes
                        .iter()
                        .next_back()
                        .expect("unexpected end of scope"),
                    shadow: Some(decl.clone()),
                });
            })
            .or_insert_with(|| {
                Rc::new(Decl {
                    declared_type,
                    scope: *self
                        .scopes
                        .iter()
                        .next_back()
                        .expect("unexpected end of scope"),
                    shadow: None,
                })
            });

        Output::Empty
    }

    fn get_typeof(&mut self, ident: Ident) -> Output {
        let mut output = Output::UnDecl;
        self.decls.entry(ident).and_modify(|decl| {
            if let Some(inner) = decl.in_scope(&self.scopes) {
                *decl = inner.clone();
                output = Output::Type(Type(decl.declared_type.0.clone()));
            }
        });

        output
    }
}

fn hash(a: &str) -> usize {
    a.chars().fold(0, |acc, c| acc * 397 + c as usize)
}

fn main() {
    let stdin = io::stdin();
    let mut qs = Quickscope::new();
    let stmts = stdin
        .lock()
        .lines()
        .skip(1)
        .map(Result::unwrap)
        .enumerate()
        .map(|(lineno, line)| Stmt::parse(lineno + 2, &line));
    print!("{}", qs.eval(stmts));
}
