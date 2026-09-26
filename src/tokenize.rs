use std::process;
use std::sync::OnceLock;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Punct(String),
    Keyword(String),
    Ident(String),
    Num(i64),
    Str(Vec<u8>),
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

pub static FILENAME: OnceLock<String> = OnceLock::new();

pub fn error_at(src: &str, pos: usize, msg: &str) -> ! {
    let line_start = src[..pos].rfind('\n').map_or(0, |i| i + 1);
    let line_end = src[pos..].find('\n').map_or(src.len(), |i| pos + i);
    let line_no = src[..pos].matches('\n').count() + 1;

    let name = FILENAME.get().map(String::as_str).unwrap_or("-");
    let prefix = format!("{}:{}: ", name, line_no);
    eprintln!("{}{}", prefix, &src[line_start..line_end]);

    let width = |s: &str| {
        s.chars()
            .map(|c| if c.is_ascii() { 1 } else { 2 })
            .sum::<usize>()
    };
    let indent = width(&prefix) + width(&src[line_start..pos]);
    eprintln!("{}^ {}", " ".repeat(indent), msg);
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

        if c == b'"' {
            let start = i;
            i += 1;
            let mut bytes = Vec::new();
            loop {
                if i >= s.len() {
                    error_at(src, start, "string literal is not closed");
                }
                match s[i] {
                    b'"' => break,
                    b'\\' => {
                        i += 1;
                        if i >= s.len() {
                            error_at(src, start, "string literal is not closed");
                        }
                        bytes.push(match s[i] {
                            b'n' => b'\n',
                            b't' => b'\t',
                            b'0' => 0,
                            other => other,
                        });
                    }
                    b => bytes.push(b),
                }
                i += 1;
            }
            i += 1;
            toks.push(Token {
                kind: TokenKind::Str(bytes),
                pos: start,
            });
            continue;
        }
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
