use std::process;
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Punct(String),
    Keyword(String),
    Ident(String),
    Num(i64),
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

const KEYWORDS: [&str; 8] = [
    "return", "if", "else", "while", "for", "int", "sizeof", "char",
];

pub fn error_at(src: &str, pos: usize, msg: &str) -> ! {
    eprintln!("{}", src);
    eprintln!("{}^ {}", " ".repeat(pos), msg);
    process::exit(1);
}

fn is_ident1(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_ident2(c: u8) -> bool {
    is_ident1(c) || c.is_ascii_digit()
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
        if is_ident1(c) {
            let start = i;
            while i < s.len() && is_ident2(s[i]) {
                i += 1;
            }
            let word = src[start..i].to_string();
            let kind = if KEYWORDS.contains(&word.as_str()) {
                TokenKind::Keyword(word)
            } else {
                TokenKind::Ident(word)
            };
            toks.push(Token { kind, pos: start });
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
        if b"+-*/()<>=;{},&[]".contains(&c) {
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
