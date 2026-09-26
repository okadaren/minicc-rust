use crate::{
    parse::{BinOp, Function, Node, Program},
    types::{Type, type_of},
};

const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
const ARG_REGS32: [&str; 6] = ["edi", "esi", "edx", "ecx", "r8d", "r9d"];
const ARG_REGS8: [&str; 6] = ["dil", "sil", "dl", "cl", "r8b", "r9b"];

struct Codegen {
    label: usize, // ラベルの通し番号
    depth: usize, // push してまだ pop していない数
    func: String,
}

impl Codegen {
    fn push(&mut self) {
        println!("  push rax");
        self.depth += 1;
    }

    fn pop(&mut self, reg: &str) {
        println!("  pop {}", reg);
        self.depth -= 1;
    }

    fn new_label(&mut self) -> usize {
        self.label += 1;
        self.label
    }

    fn gen_addr(&mut self, node: &Node) {
        match node {
            Node::Var { offset, .. } => println!("  lea rax, [rbp-{}]", offset),
            Node::GVar { name, .. } => println!("   lea rax, [rip+{}]", name),
            Node::Deref(e) => self.gen_expr(e),
            _ => unreachable!(),
        }
    }

    fn load(&self, ty: &Type) {
        match ty {
            Type::Char => println!("  movsx rax, byte ptr [rax]"),
            Type::Int => println!("  movsxd rax, dword ptr [rax]"),
            Type::Ptr(_) => println!("  mov rax, [rax]"),
            Type::Array(..) => {}
        }
    }

    fn store(&mut self, ty: &Type) {
        self.pop("rdi");
        match ty.size() {
            1 => println!(" mov [rdi], al"),
            4 => println!("  mov [rdi], eax"),
            _ => println!("  mov [rdi], rax"),
        }
    }

    fn gen_expr(&mut self, node: &Node) {
        match node {
            Node::Num(n) => println!("  mov rax, {}", n),
            Node::Var { ty, .. } | Node::GVar { ty, .. } => {
                self.gen_addr(node);
                self.load(ty);
            }
            Node::Addr(e) => self.gen_addr(e),
            Node::Deref(e) => {
                self.gen_expr(e);
                self.load(&type_of(node));
            }
            Node::Assign(lhs, rhs) => {
                self.gen_addr(lhs);
                self.push();
                self.gen_expr(rhs);
                self.store(&type_of(lhs));
            }
            Node::Call(name, args) => {
                for arg in args {
                    self.gen_expr(arg);
                    self.push();
                }
                for i in (0..args.len()).rev() {
                    self.pop(ARG_REGS[i]);
                }
                if self.depth % 2 == 1 {
                    println!("  sub rsp, 8");
                }
                println!("  mov rax, 0");
                println!("  call {}", name);
                if self.depth % 2 == 1 {
                    println!("  add rsp, 8");
                }
                println!("  movsxd rax, eax");
            }
            Node::Binary(op, lhs, rhs) => {
                self.gen_expr(rhs);
                self.push();
                self.gen_expr(lhs);
                self.pop("rdi");
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
            }
            Node::Return(_) | Node::If { .. } | Node::For { .. } | Node::Block(_) => unreachable!(),
        };
    }

    fn gen_stmt(&mut self, node: &Node) {
        match node {
            Node::Return(e) => {
                self.gen_expr(e);
                println!("  jmp .L.return.{}", self.func);
            }
            Node::If { cond, then, els } => {
                let c = self.new_label();
                self.gen_expr(cond);
                println!("  cmp rax, 0");
                println!("  je .L.else.{}", c);
                self.gen_stmt(then);
                println!("  jmp .L.end.{}", c);
                println!(".L.else.{}:", c);
                if let Some(e) = els {
                    self.gen_stmt(e);
                }
                println!(".L.end.{}:", c);
            }
            Node::For {
                init,
                cond,
                inc,
                body,
            } => {
                let c = self.new_label();
                if let Some(i) = init {
                    self.gen_expr(i);
                }
                println!(".L.begin.{}:", c);
                if let Some(cd) = cond {
                    self.gen_expr(cd);
                    println!("  cmp rax, 0");
                    println!("  je .L.end.{}", c);
                }
                self.gen_stmt(body);
                if let Some(i) = inc {
                    self.gen_expr(i);
                }
                println!("  jmp .L.begin.{}", c);
                println!(".L.end.{}:", c);
            }
            Node::Block(stmts) => {
                for s in stmts {
                    self.gen_stmt(s);
                }
            }
            _ => self.gen_expr(node),
        }
    }

    fn gen_function(&mut self, f: &Function) {
        self.func = f.name.clone();

        println!(".globl {}", f.name);
        println!("{}:", f.name);

        println!("  push rbp");
        println!("  mov rbp, rsp");
        println!("  sub rsp, {}", f.stack_size);

        for (i, (offset, ty)) in f.params.iter().enumerate() {
            let reg = match ty.size() {
                1 => ARG_REGS8[i],
                4 => ARG_REGS32[i],
                _ => ARG_REGS[i],
            };
            println!("  mov [rbp-{}], {}", offset, reg);
        }

        for stmt in &f.body {
            self.gen_stmt(stmt);
        }
        assert_eq!(self.depth, 0);

        println!(".L.return.{}:", f.name);
        println!("  mov rsp, rbp");
        println!("  pop rbp");
        println!("  ret");
    }
}

pub fn gen_program(prog: &Program) {
    let mut cg = Codegen {
        label: 0,
        depth: 0,
        func: String::new(),
    };

    println!(".intel_syntax noprefix");

    println!(".data");
    for g in &prog.globals {
        println!(".globl {}", g.name);
        println!("{}:", g.name);
        println!("  .zero {}", g.ty.size());
    }

    println!(".text");
    for f in &prog.funcs {
        cg.gen_function(f);
    }
}
