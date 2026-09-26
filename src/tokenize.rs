use std::process;

use crate::tokenize::TokenKind::Ident;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Punct(String),
    Ident(char),
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
        if c.is_ascii_lowercase() {
            toks.push(Token {
                kind: Ident(c as char),
                pos: i,
            });
            i += 1;
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
        if b"+-*/()<>=;".contains(&c) {
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
