mod codegen;
mod parse;
mod tokenize;
mod types;

use std::io::Read;
use std::{env, fs, io, process};

fn read_file(path: &str) -> String {
    let mut src = if path == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
            eprintln!("can't read stdin: {}", e);
            process::exit(1);
        });
        buf
    } else {
        fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("{} can't read: {}", path, e);
            process::exit(1);
        })
    };

    if !src.ends_with('\n') {
        src.push('\n');
    }
    src
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: rcc <file>");
        process::exit(1);
    }
    let path = &args[1];
    tokenize::FILENAME.set(path.clone()).unwrap();

    let src = read_file(path);
    let prog = parse::Parser::new(&src).program();
    codegen::gen_program(&prog);
}
