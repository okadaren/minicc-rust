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
        if b"+-".contains(&c) {
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

struct Cursor<'a> {
    src: &'a str,
    toks: Vec<Token>,
    pos: usize,
}

impl<'a> Cursor<'a> {
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: rcc <code>");
        process::exit(1);
    }
    let src = &args[1];
    let mut cur = Cursor {
        src,
        toks: tokenize(src),
        pos: 0,
    };

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // 式の最初は数でなければならない
    println!("  mov rax, {}", cur.expect_number());

    while !cur.at_eof() {
        if cur.consume("+") {
            println!("  add rax, {}", cur.expect_number());
            continue;
        }
        cur.expect("-");
        println!("  sub rax, {}", cur.expect_number());
    }

    println!("  ret");
}
