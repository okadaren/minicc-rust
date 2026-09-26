use crate::parse::{BinOp, Node};

fn gen_lval(node: &Node) {
    match node {
        Node::Var(offset) => {
            println!("  lea rax, [rbp-{}]", offset);
            println!("  push rax");
        }
        _ => unreachable!(),
    }
}

fn gen_expr(node: &Node) {
    let (op, lhs, rhs) = match node {
        Node::Num(n) => {
            println!("  push {}", n);
            return;
        }
        Node::Var(_) => {
            gen_lval(node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        Node::Assign(lhs, rhs) => {
            gen_lval(lhs);
            gen_expr(rhs);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        }
        Node::Binary(op, lhs, rhs) => (op, lhs, rhs),
    };

    gen_expr(lhs);
    gen_expr(rhs);

    println!("  pop rdi");
    println!("  pop rax");

    match op {
        BinOp::Add => println!("  add rax, rdi"),
        BinOp::Sub => println!("  sub rax, rdi"),
        BinOp::Mul => println!("  imul rax, rdi"),
        BinOp::Div => {
            println!("  cqo\n");
            println!("  idiv rdi\n");
        }
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le => {
            let set = match op {
                BinOp::Eq => "sete",
                BinOp::Ne => "setne",
                BinOp::Lt => "setl",
                _ => "setle",
            };
            println!("  cmp rax, rdi");
            println!("  {} al", set);
            println!("  movzb rax, al");
        }
    }

    println!("  push rax\n")
}

pub fn gen_program(stmts: &[Node]) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    for stmt in stmts {
        gen_expr(stmt);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
