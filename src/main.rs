use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: rcc <code>");
        process::exit(1);
    }
    let n: i64 = args[1].trim().parse().expect("整数ではありません");
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", n);
    println!("  ret");
}
