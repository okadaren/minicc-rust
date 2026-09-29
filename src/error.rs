use std::fmt;

#[must_use]
#[derive(Debug, Clone)]
pub struct CompileError {
    pub pos: usize,
    pub msg: String,
}

impl CompileError {
    pub fn report(&self, filename: &str, src: &str) {
        let pos = self.pos;
        let line_start = src[..pos].rfind('\n').map_or(0, |i| i + 1);
        let line_end = src[pos..].find('\n').map_or(src.len(), |i| pos + i);
        let line_no = src[..pos].matches('\n').count() + 1;

        let prefix = format!("{}:{}: ", filename, line_no);
        eprintln!("{}{}", prefix, &src[line_start..line_end]);

        let width = |s: &str| {
            s.chars()
                .map(|c| if c.is_ascii() { 1 } else { 2 })
                .sum::<usize>()
        };
        let indent = width(&prefix) + width(&src[line_start..pos]);
        eprintln!("{}^ {}", " ".repeat(indent), self.msg);
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl std::error::Error for CompileError {}

pub type Result<T> = std::result::Result<T, CompileError>;

pub fn error_at<T>(pos: usize, msg: impl Into<String>) -> Result<T> {
    Err(CompileError {
        pos,
        msg: msg.into(),
    })
}
