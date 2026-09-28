mod codegen;
mod error;
mod parse;
mod tokenize;
mod types;

use std::io::Read;
use std::process::ExitCode;
use std::{env, fs, io};

use crate::error::Result;
use crate::parse::{Parser, Program};

fn read_file(path: &str) -> io::Result<String> {
    let mut src = if path == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        fs::read_to_string(path)?
    };

    if !src.ends_with('\n') {
        src.push('\n');
    }
    Ok(src)
}

fn compile(src: &str) -> Result<Program> {
    Parser::new(src)?.program()
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let [_, path] = args.as_slice() else {
        eprintln!("usage: rcc <file>");
        return ExitCode::FAILURE;
    };
    let src = match read_file(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {}", path, e);
            return ExitCode::FAILURE;
        }
    };

    match compile(&src) {
        Ok(prog) => {
            codegen::gen_program(&prog);
            ExitCode::SUCCESS
        }
        Err(e) => {
            e.report(path, &src);
            ExitCode::FAILURE
        }
    }
}
