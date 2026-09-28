use std::fmt;

use crate::error::{Result, error_at};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punct {
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    Eq,       // ==
    Ne,       // !=
    Lt,       // <
    Le,       // <=
    Gt,       // >
    Ge,       // >=
    Assign,   // =
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]
    Semi,     // ;
    Comma,    // ,
    Amp,      // &
}

impl Punct {
    pub const ALL: [Punct; 20] = [
        Punct::Eq,
        Punct::Ne,
        Punct::Le,
        Punct::Ge,
        Punct::Plus,
        Punct::Minus,
        Punct::Star,
        Punct::Slash,
        Punct::Lt,
        Punct::Gt,
        Punct::Assign,
        Punct::LParen,
        Punct::RParen,
        Punct::LBrace,
        Punct::RBrace,
        Punct::LBracket,
        Punct::RBracket,
        Punct::Semi,
        Punct::Comma,
        Punct::Amp,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Punct::Plus => "+",
            Punct::Minus => "-",
            Punct::Star => "*",
            Punct::Slash => "/",
            Punct::Eq => "==",
            Punct::Ne => "!=",
            Punct::Lt => "<",
            Punct::Le => "<=",
            Punct::Gt => ">",
            Punct::Ge => ">=",
            Punct::Assign => "=",
            Punct::LParen => "(",
            Punct::RParen => ")",
            Punct::LBrace => "{",
            Punct::RBrace => "}",
            Punct::LBracket => "[",
            Punct::RBracket => "]",
            Punct::Semi => ";",
            Punct::Comma => ",",
            Punct::Amp => "&",
        }
    }
}

impl fmt::Display for Punct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Return,
    If,
    Else,
    While,
    For,
    Int,
    Char,
    Sizeof,
}

impl Keyword {
    pub const ALL: [Keyword; 8] = [
        Keyword::Return,
        Keyword::If,
        Keyword::Else,
        Keyword::While,
        Keyword::For,
        Keyword::Int,
        Keyword::Char,
        Keyword::Sizeof,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Keyword::Return => "return",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::For => "for",
            Keyword::Int => "int",
            Keyword::Char => "char",
            Keyword::Sizeof => "sizeof",
        }
    }

    pub fn lookup(word: &str) -> Option<Keyword> {
        Keyword::ALL.into_iter().find(|kw| kw.as_str() == word)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Punct(Punct),
    Keyword(Keyword),
    Ident(String),
    Num(i64),
    Str(Vec<u8>),
    Eof,
}

impl From<Punct> for TokenKind {
    fn from(p: Punct) -> Self {
        TokenKind::Punct(p)
    }
}

impl From<Keyword> for TokenKind {
    fn from(k: Keyword) -> Self {
        TokenKind::Keyword(k)
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

fn is_ident1(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_'
}

fn is_ident2(c: u8) -> bool {
    is_ident1(c) || c.is_ascii_digit()
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
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
                    return error_at(start, "unterminated string literal");
                }
                match s[i] {
                    b'"' => break,
                    b'\\' => {
                        i += 1;
                        if i >= s.len() {
                            return error_at(start, "unterminated string literal");
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
        if src[i..].starts_with("//") {
            i = src[i..].find('\n').map_or(s.len(), |n| i + n);
            continue;
        }
        if src[i..].starts_with("/*") {
            match src[i + 2..].find("*/") {
                Some(n) => i = i + 2 + n + 2,
                None => return error_at(i, "unterminated comment"),
            }
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
            let word = &src[start..i];
            let kind = match Keyword::lookup(word) {
                Some(kw) => TokenKind::Keyword(kw),
                None => TokenKind::Ident(word.to_string()),
            };
            toks.push(Token { kind, pos: start });
            continue;
        }

        if let Some(&p) = Punct::ALL.iter().find(|p| src[i..].starts_with(p.as_str())) {
            toks.push(Token {
                kind: TokenKind::Punct(p),
                pos: i,
            });
            i += p.as_str().len();
            continue;
        }
        let c = src[i..].chars().next().unwrap();
        return error_at(i, &format!("unexpected character '{}'", c));
    }
    toks.push(Token {
        kind: TokenKind::Eof,
        pos: i,
    });
    Ok(toks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number() {
        let toks = tokenize("42").unwrap();
        assert_eq!(toks[0].kind, TokenKind::Num(42));
        assert_eq!(toks[1].kind, TokenKind::Eof);
    }

    fn assert_error(marked: &str, msg: &str) {
        let pos = marked.find('^').expect("no ^ marker");
        let src = marked.replacen('^', "", 1);
        let err = tokenize(&src).expect_err(msg);
        assert_eq!((err.pos, err.msg.as_str()), (pos, msg), "src: {}", src);
    }

    #[test]
    fn unterminated_string() {
        assert_error("return ^\"abc", "unterminated string literal");
    }

    #[test]
    fn unterminated_string_after_backslash() {
        assert_error("return ^\"abc\\", "unterminated string literal");
    }

    #[test]
    fn unterminated_comment() {
        assert_error("int x; ^/* comment", "unterminated comment");
    }

    #[test]
    fn unexpected_character() {
        assert_error("1 + ^@", "unexpected character '@'");
    }

    #[test]
    fn unexpected_non_ascii_character() {
        assert_error("int ^あ;", "unexpected character 'あ'");
    }

    #[test]
    fn every_punct_is_tokenized() {
        for p in Punct::ALL {
            let toks = tokenize(p.as_str()).unwrap();
            assert_eq!(toks[0].kind, TokenKind::Punct(p), "{}", p.as_str());
            assert_eq!(toks[1].kind, TokenKind::Eof, "{}", p.as_str());
        }
    }

    #[test]
    fn keyword_and_ident() {
        let toks = tokenize("return returnx").unwrap();
        assert_eq!(toks[0].kind, TokenKind::Keyword(Keyword::Return));
        assert_eq!(toks[1].kind, TokenKind::Ident("returnx".to_string()));
    }
}
