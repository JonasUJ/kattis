use std::{
    collections::HashSet,
    io::{self, BufRead},
    iter::Peekable,
    slice::Iter,
};

#[derive(PartialEq, Debug)]
enum Operator {
    And,
    Or,
    Not,
}

#[derive(PartialEq, Debug)]
enum Token {
    If,
    Then,
    Else,
    Fi,
    Checkpoint,
    LParen,
    RParen,
    Op(Operator),
    Var(char),
}

trait Expr {
    fn process(&self) -> Option<HashSet<char>>;
}

struct And {
    lhs: Box<dyn Expr>,
    rhs: Box<dyn Expr>,
}

struct Or {
    lhs: Box<dyn Expr>,
    rhs: Box<dyn Expr>,
}

struct Not {
    expr: Box<dyn Expr>,
}

struct Var {
    name: char,
}

impl Expr for And {
    fn process(&self) -> Option<HashSet<char>> {
        let left = self.lhs.process()?;
        let right = self.rhs.process()?;

        and(&left, &right)
    }
}

impl Expr for Or {
    fn process(&self) -> Option<HashSet<char>> {
        let left = self.lhs.process();
        let right = self.rhs.process();

        if let None = left {
            return right;
        } else if let None = right {
            return left;
        }

        // Safe to unwrap because we just check if either was None.
        let left = left.unwrap();
        let right = right.unwrap();

        Some(left.union(&right).cloned().collect())
    }
}

impl Expr for Not {
    fn process(&self) -> Option<HashSet<char>> {
        Some(self.expr.process()?.iter().map(invert_case).collect())
    }
}

impl Expr for Var {
    fn process(&self) -> Option<HashSet<char>> {
        Some([self.name].iter().cloned().collect())
    }
}

trait Node {
    fn process(&self) -> Vec<Option<HashSet<char>>>;
    fn _debug(&self, i: &usize);
}

struct IfStmt {
    expr: Box<dyn Expr>,
    true_nodes: Vec<Box<dyn Node>>,
    false_nodes: Vec<Box<dyn Node>>,
}

struct CheckpointStmt;

impl Node for IfStmt {
    fn process(&self) -> Vec<Option<HashSet<char>>> {
        let mut v: Vec<Option<HashSet<char>>> = vec![];
        let res = self.expr.process();

        // Add to v the result of combining self.expr with every node in true_nodes and inverted
        // self.expr with every node in false_nodes. If either is None, add None instead.
        let mut process_nodes = |iter: Iter<'_, Box<dyn Node>>, invert: bool| {
            for node in iter {
                for inner_set_option in node.process() {
                    if let Some(ref outer_set) = res {
                        if let Some(inner_set) = inner_set_option {
                            v.push(and(
                                &outer_set
                                    .iter()
                                    .map(|c| if invert { invert_case(c) } else { *c })
                                    .collect(),
                                &inner_set,
                            ))
                        } else {
                            v.push(None)
                        }
                    } else {
                        v.push(None)
                    }
                }
            }
        };

        process_nodes(self.true_nodes.iter(), false);
        process_nodes(self.false_nodes.iter(), true);

        v
    }
    fn _debug(&self, i: &usize) {
        println!("{:i$}if", " ", i=i);
        let i = i + 2;
        for node in self.true_nodes.iter() {
            node._debug(&i);
        }
        for node in self.false_nodes.iter() {
            node._debug(&i);
        }
    }
}

impl Node for CheckpointStmt {
    fn process(&self) -> Vec<Option<HashSet<char>>> {
        let mut v = Vec::<Option<HashSet<char>>>::new();
        v.push(Some(HashSet::new()));
        v
    }
    fn _debug(&self, i: &usize) {
        println!("{:i$}checkpoint", " ", i=i);
    }
}

struct AST {
    nodes: Vec<Box<dyn Node>>,
}

impl AST {
    fn check(&self) -> Vec<String> {
        let mut v: Vec<Option<HashSet<char>>> = vec![];

        for node in self.nodes.iter() {
            v.extend(node.process());
        }

        v.iter()
            .map(|s| match s {
                Some(set) => {
                    let mut s = String::with_capacity(set.len());
                    let mut v: Vec<char> = set.iter().cloned().collect();
                    v.sort_by_cached_key(char::to_ascii_uppercase);
                    s.extend(v);
                    s
                }
                None => String::from("unreachable"),
            })
            .collect()
    }
}

fn main() {
    // let source = io::stdin()
    //     .lock()
    //     .lines()
    //     .map(Result::unwrap)
    //     .collect::<Vec<String>>()
    //     .join(" ");
    let source = String::from("if (A&B | ~A&~B) & (~A&B) then checkpoint fi");
    let tokens = lex(&source);
    println!("{:?}", tokens);
    let ast = parse(tokens);
    let result = ast.check();
    for s in result {
        println!(">{}", s);
    }
    for node in ast.nodes {
        node._debug(&0);
    }
}

fn invert_case(c: &char) -> char {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

fn clean(set: &HashSet<char>) -> HashSet<char> {
    let mut new = set.clone();
    new.retain(|c| !set.contains(&invert_case(c)));
    new
}

fn and(a: &HashSet<char>, b: &HashSet<char>) -> Option<HashSet<char>> {
    // If a var is present as true on one side and as false on the other,
    // then the And is unsolveable.
    if a.is_disjoint(&b.iter().map(invert_case).collect()) {
        Some(clean(&a.union(&b).cloned().collect()))
    } else {
        None
    }
}

fn lex(source: &String) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    // Checkpoint is the longest token.
    let mut cur = String::with_capacity("checkpoint".len());

    for c in source.chars() {
        if c == ' ' {
            if cur.len() > 0 {
                tokens.push(parse_token(&cur));
                cur.clear();
            }
        } else if c.is_uppercase() || !c.is_ascii_alphabetic() {
            tokens.push(parse_token(&c.to_string()));
        } else {
            cur.push(c);
        }
    }

    tokens.push(parse_token(&cur));

    tokens
}

fn parse_token(s: &String) -> Token {
    match s.as_str() {
        "if" => Token::If,
        "then" => Token::Then,
        "else" => Token::Else,
        "fi" => Token::Fi,
        "checkpoint" => Token::Checkpoint,
        "(" => Token::LParen,
        ")" => Token::RParen,
        "&" => Token::Op(Operator::And),
        "|" => Token::Op(Operator::Or),
        "~" => Token::Op(Operator::Not),

        // Vars are guarenteed to be 1 char.
        _ => Token::Var(
            s.chars()
                .next()
                .expect("Expected variable name, but nothing was found."),
        ),
    }
}

fn parse(tokens: Vec<Token>) -> AST {
    let mut nodes = Vec::<Box<dyn Node>>::new();
    let mut iter = tokens.iter().peekable();

    while let Some(tok) = iter.next() {
        match tok {
            &Token::Checkpoint => nodes.push(Box::new(CheckpointStmt)),
            &Token::If => nodes.push(parse_if(&mut iter)),

            // Syntax error
            err => panic!("Unexpected '{:?}' while parsing statements.", err),
        }
    }

    AST { nodes }
}

fn parse_if(iter: &mut Peekable<Iter<'_, Token>>) -> Box<dyn Node> {
    // parse_if is only called if the Token::If has already been consumed.
    let expr = parse_expr(iter);

    // Expect Then after expression.
    assert_eq!(iter.next(), Option::Some(&Token::Then));

    let mut true_nodes = Vec::<Box<dyn Node>>::new();
    let mut false_nodes = Vec::<Box<dyn Node>>::new();

    let mut nodes = &mut true_nodes;

    while let Some(tok) = iter.next() {
        match tok {
            &Token::Checkpoint => nodes.push(Box::new(CheckpointStmt)),
            &Token::If => nodes.push(parse_if(iter)),
            &Token::Else => nodes = &mut false_nodes,
            &Token::Fi => break,

            // Syntax error
            err => panic!("Unexpected '{:?}' while parsing statements.", err),
        }
    }

    Box::new(IfStmt {
        expr,
        true_nodes,
        false_nodes,
    })
}

fn parse_expr(iter: &mut Peekable<Iter<'_, Token>>) -> Box<dyn Expr> {
    let mut expr: Option<Box<dyn Expr>> = None;

    while let Some(tok) = iter.next() {
        expr = Some(match tok {
            &Token::LParen => parse_expr(iter),
            &Token::RParen => return expr.expect("Unexpected ')' while parsing expression."),
            &Token::Var(name) => Box::new(Var { name }),
            &Token::Op(Operator::Not) => Box::new(Not {
                expr: parse_expr(iter),
            }),
            &Token::Op(Operator::And) => Box::new(And {
                lhs: expr.expect("Unexpected '&' while parsing expression."),
                rhs: parse_expr(iter),
            }),
            &Token::Op(Operator::Or) => {
                let tmp = parse_expr(iter);

                // Simple operator precedence parser because we only have two kinds of binary
                // operators to deal with.
                if let Some(&Token::Op(Operator::And)) = iter.peek() {
                    Box::new(Or {
                        lhs: expr.expect("Unexpected '|' while parsing expression."),
                        rhs: Box::new(And {
                            lhs: tmp,
                            rhs: parse_expr(iter),
                        }),
                    })
                } else {
                    Box::new(Or {
                        lhs: expr.expect("Unexpected '|' while parsing expression."),
                        rhs: tmp,
                    })
                }
            }

            _ => panic!("Unexpected '{:?}' while parsing expression.", tok),
        });

        if let Some(&Token::Then) = iter.peek() {
            return expr.expect("Unexpected end of expression while parsing expression.");
        }
    }

    // Syntax error
    panic!("EOF while parsing expression.");
}
