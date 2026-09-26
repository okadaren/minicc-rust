use crate::parse::{BinOp, Node, Program};

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
        Node::Return(_) | Node::If { .. } | Node::For { .. } | Node::Block(_) => unreachable!(),
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

fn gen_discard(node: &Node) {
    gen_expr(node);
    println!("  pop rax");
}

fn gen_stmt(node: &Node, label: &mut usize) {
    match node {
        Node::Return(e) => {
            gen_expr(e);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret")
        }
        Node::If { cond, then, els } => {
            *label += 1;
            let c = *label;
            gen_expr(cond);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je .Lelse{}", c);
            gen_stmt(then, label);
            println!("  jmp .Lend{}", c);
            println!(".Lelse{}:", c);
            if let Some(e) = els {
                gen_stmt(e, label);
            }
            println!(".Lend{}:", c);
        }
        Node::For {
            init,
            cond,
            inc,
            body,
        } => {
            *label += 1;
            let c = *label;
            if let Some(i) = init {
                gen_discard(i);
            }
            println!(".Lbegin{}:", c);
            if let Some(cd) = cond {
                gen_expr(cd);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je .Lend{}", c);
            }
            gen_stmt(body, label);
            if let Some(i) = inc {
                gen_discard(i);
            }
            println!("  jmp .Lbegin{}", c);
            println!(".Lend{}:", c);
        }
        Node::Block(stmts) => {
            for s in stmts {
                gen_stmt(s, label);
            }
        }
        _ => gen_discard(node),
    }
}

pub fn gen_program(prog: &Program) {
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", prog.stack_size);

    let mut label = 0;
    for stmt in &prog.body {
        gen_stmt(stmt, &mut label);
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
