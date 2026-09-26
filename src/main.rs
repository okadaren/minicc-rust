mod codegen;
mod parse;
mod tokenize;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: rcc <code>");
        process::exit(1);
    }
    let src = &args[1];

    let prog = parse::Parser::new(src).program();
    codegen::gen_program(&prog);
}
