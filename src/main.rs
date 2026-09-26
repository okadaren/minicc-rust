use std::{env, process};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Punct(String),
    Num(i64),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

pub fn error_at(src: &str, pos: usize, msg: &str) -> ! {
    eprintln!("{}", src);
    eprintln!("{}^ {}", " ".repeat(pos), msg);
    process::exit(1);
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let s = src.as_bytes();
    let mut toks = Vec::new();
    let mut i = 0;
    while i < s.len() {
        let c = s[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c.is_ascii_digit() {
            let start = i;
            while i < s.len() && s[i].is_ascii_digit() {
                i += 1;
            }
            let n = src[start..i].parse().unwrap();
            toks.push(Token {
                kind: TokenKind::Num(n),
                pos: start,
            });
            continue;
        }
        // 2文字の記号
        let rest = &src[i..];
        if let Some(op) = ["==", "!=", "<=", ">="]
            .iter()
            .find(|op| rest.starts_with(**op))
        {
            toks.push(Token {
                kind: TokenKind::Punct(op.to_string()),
                pos: i,
            });
            i += 2;
            continue;
        }
        if b"+-*/()<>".contains(&c) {
            toks.push(Token {
                kind: TokenKind::Punct((c as char).to_string()),
                pos: i,
            });
            i += 1;
            continue;
        }
        error_at(src, i, "can't tokenize");
    }
    toks.push(Token {
        kind: TokenKind::Eof,
        pos: i,
    });
    toks
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
}

#[derive(Debug)]
pub enum Node {
    Num(i64),
    Binary(BinOp, Box<Node>, Box<Node>),
}

fn bin(op: BinOp, l: Node, r: Node) -> Node {
    Node::Binary(op, Box::new(l), Box::new(r))
}

struct Parser<'a> {
    src: &'a str,
    toks: Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        Parser {
            src,
            toks: tokenize(src),
            pos: 0,
        }
    }

    fn peek(&self) -> &Token {
        &self.toks[self.pos]
    }

    // 次が記号なら読み進めてtrueを返す
    fn consume(&mut self, op: &str) -> bool {
        if matches!(&self.peek().kind, TokenKind::Punct(s) if s == op) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    // 次が記号でなければエラー
    fn expect(&mut self, op: &str) {
        if !self.consume(op) {
            error_at(self.src, self.peek().pos, &format!("not '{}'", op));
        }
    }

    // 次が整数なら読み進めてその値を返す。そうでなければエラー
    fn expect_number(&mut self) -> i64 {
        match self.peek().kind {
            TokenKind::Num(n) => {
                self.pos += 1;
                n
            }
            _ => error_at(self.src, self.peek().pos, "not number"),
        }
    }

    fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    fn expr(&mut self) -> Node {
        self.equality()
    }

    fn equality(&mut self) -> Node {
        let mut node = self.relational();

        loop {
            if self.consume("==") {
                node = bin(BinOp::Eq, node, self.relational());
            } else if self.consume("!=") {
                node = bin(BinOp::Ne, node, self.relational());
            } else {
                return node;
            }
        }
    }

    fn relational(&mut self) -> Node {
        let mut node = self.add();

        loop {
            if self.consume("<") {
                node = bin(BinOp::Lt, node, self.add());
            } else if self.consume("<=") {
                node = bin(BinOp::Le, node, self.add());
            } else if self.consume(">") {
                node = bin(BinOp::Lt, self.add(), node);
            } else if self.consume(">=") {
                node = bin(BinOp::Le, self.add(), node);
            } else {
                return node;
            }
        }
    }

    fn add(&mut self) -> Node {
        let mut node = self.mul();

        loop {
            if self.consume("+") {
                node = bin(BinOp::Add, node, self.mul());
            } else if self.consume("-") {
                node = bin(BinOp::Sub, node, self.mul());
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Node {
        let mut node = self.unary();

        loop {
            if self.consume("*") {
                node = bin(BinOp::Mul, node, self.unary())
            } else if self.consume("/") {
                node = bin(BinOp::Div, node, self.unary())
            } else {
                return node;
            }
        }
    }

    fn unary(&mut self) -> Node {
        if self.consume("+") {
            return self.unary();
        }
        if self.consume("-") {
            return bin(BinOp::Sub, Node::Num(0), self.unary());
        }
        self.primary()
    }

    fn primary(&mut self) -> Node {
        if self.consume("(") {
            let node = self.expr();
            self.expect(")");
            return node;
        }
        Node::Num(self.expect_number())
    }
}

fn gen_expr(node: &Node) {
    let (op, lhs, rhs) = match node {
        Node::Num(n) => {
            println!("  push {}", n);
            return;
        }
        Node::Binary(op, lhs, rhs) => (op, lhs, rhs),
    };

    gen_expr(lhs);
    gen_expr(rhs);

    println!("  pop rdi");
    println!("  pop rax");

    match op {
        BinOp::Add => println!("  add rax, rdi"),
        BinOp::Sub => println!("  sub rax, rdi"),
        BinOp::Mul => println!("  imul rax, rdi"),
        BinOp::Div => {
            println!("  cqo\n");
            println!("  idiv rdi\n");
        }
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le => {
            let set = match op {
                BinOp::Eq => "sete",
                BinOp::Ne => "setne",
                BinOp::Lt => "setl",
                _ => "setle",
            };
            println!("  cmp rax, rdi");
            println!("  {} al", set);
            println!("  movzb rax, al");
        }
    }

    println!("  push rax\n")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: rcc <code>");
        process::exit(1);
    }
    let src = &args[1];
    let mut parser = Parser::new(src);
    let node = parser.expr();
    if !parser.at_eof() {
        error_at(src, parser.peek().pos, "extra tokens")
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen_expr(&node);

    println!("  pop rax");
    println!("  ret")
}
