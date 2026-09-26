use crate::tokenize::{Token, TokenKind, error_at, tokenize};
use crate::types::{Type, type_of};

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
}

#[derive(Debug)]
pub enum Node {
    Num(i64),
    Var {
        offset: i64,
        ty: Type,
    },
    Binary(BinOp, Box<Node>, Box<Node>),
    Assign(Box<Node>, Box<Node>),
    Return(Box<Node>),
    If {
        cond: Box<Node>,
        then: Box<Node>,
        els: Option<Box<Node>>,
    },
    For {
        init: Option<Box<Node>>,
        cond: Option<Box<Node>>,
        inc: Option<Box<Node>>,
        body: Box<Node>,
    },
    Block(Vec<Node>),
    Call(String, Vec<Node>),
    Addr(Box<Node>),
    Deref(Box<Node>),
}

struct LVar {
    name: String,
    offset: i64,
    ty: Type,
}

pub struct Function {
    pub name: String,
    pub params: Vec<i64>,
    pub body: Vec<Node>,
    pub stack_size: i64,
}

pub struct Program {
    pub funcs: Vec<Function>,
}

fn bin(op: BinOp, l: Node, r: Node) -> Node {
    Node::Binary(op, Box::new(l), Box::new(r))
}

pub struct Parser<'a> {
    src: &'a str,
    toks: Vec<Token>,
    pos: usize,
    locals: Vec<LVar>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            src,
            toks: tokenize(src),
            pos: 0,
            locals: Vec::new(),
        }
    }

    pub fn peek(&self) -> &Token {
        &self.toks[self.pos]
    }

    // 次が記号なら読み進めてtrueを返す
    fn consume(&mut self, op: &str) -> bool {
        if matches!(&self.peek().kind, TokenKind::Punct(s) | TokenKind::Keyword(s) if s == op) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    // 次が記号でなければエラー
    fn expect(&mut self, op: &str) {
        if !self.consume(op) {
            error_at(self.src, self.peek().pos, &format!("not '{}'", op));
        }
    }

    // 次が整数なら読み進めてその値を返す。そうでなければエラー
    fn expect_number(&mut self) -> i64 {
        match self.peek().kind {
            TokenKind::Num(n) => {
                self.pos += 1;
                n
            }
            _ => error_at(self.src, self.peek().pos, "not number"),
        }
    }

    fn consume_ident(&mut self) -> Option<String> {
        if let TokenKind::Ident(name) = &self.peek().kind {
            let name = name.clone();
            self.pos += 1;
            Some(name)
        } else {
            None
        }
    }

    fn expect_ident(&mut self, what: &str) -> String {
        let pos = self.peek().pos;
        self.consume_ident()
            .unwrap_or_else(|| error_at(self.src, pos, &format!("not {}", what)))
    }

    fn parse_type(&mut self) -> Type {
        self.expect("int");
        let mut ty = Type::Int;
        while self.consume("*") {
            ty = Type::pointer_to(ty);
        }
        ty
    }

    fn declare_var(&mut self, name: String, ty: Type, pos: usize) -> i64 {
        if self.locals.iter().any(|v| v.name == name) {
            error_at(self.src, pos, "same name variable is defined");
        }
        let offset = (self.locals.len() as i64 + 1) * 8;
        self.locals.push(LVar { name, offset, ty });
        offset
    }

    fn find_var(&self, name: &str) -> Option<&LVar> {
        self.locals.iter().find(|v| v.name == name)
    }

    fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    pub fn program(&mut self) -> Program {
        let mut funcs = Vec::new();
        while !self.at_eof() {
            funcs.push(self.function());
        }
        Program { funcs }
    }

    fn function(&mut self) -> Function {
        self.locals.clear();

        let pos = self.peek().pos;
        self.expect("int");
        let name = self.expect_ident("function name");

        self.expect("(");
        let mut params = Vec::new();
        while !self.consume(")") {
            if !params.is_empty() {
                self.expect(",");
            }
            let ty = self.parse_type();
            let pos = self.peek().pos;
            let param = self.expect_ident("argument name");
            params.push(self.declare_var(param, ty, pos));
        }
        if params.len() > 6 {
            error_at(self.src, pos, "arguments limit is 6");
        }

        self.expect("{");
        let mut body = Vec::new();
        while !self.consume("}") {
            if self.at_eof() {
                error_at(self.src, self.peek().pos, "not '}'");
            }
            body.push(self.stmt());
        }

        let stack_size = (self.locals.len() as i64 * 8 + 15) / 16 * 16;
        Function {
            name,
            params,
            body,
            stack_size,
        }
    }

    fn stmt(&mut self) -> Node {
        if matches!(&self.peek().kind, TokenKind::Keyword(k) if k == "int") {
            let ty = self.parse_type();
            let pos = self.peek().pos;
            let name = self.expect_ident("variable name");
            self.declare_var(name, ty, pos);
            self.expect(";");
            return Node::Block(Vec::new());
        }

        if self.consume("return") {
            let node = self.expr();
            self.expect(";");
            return Node::Return(Box::new(node));
        }

        if self.consume("if") {
            self.expect("(");
            let cond = Box::new(self.expr());
            self.expect(")");
            let then = Box::new(self.stmt());
            let els = if self.consume("else") {
                Some(Box::new(self.stmt()))
            } else {
                None
            };
            return Node::If { cond, then, els };
        }

        if self.consume("while") {
            self.expect("(");
            let cond = Some(Box::new(self.expr()));
            self.expect(")");
            let body = Box::new(self.stmt());
            return Node::For {
                init: None,
                cond,
                inc: None,
                body,
            };
        }

        if self.consume("for") {
            self.expect("(");
            let init = self.opt_expr(";").map(Box::new);
            let cond = self.opt_expr(";").map(Box::new);
            let inc = self.opt_expr(")").map(Box::new);
            let body = Box::new(self.stmt());
            return Node::For {
                init,
                cond,
                inc,
                body,
            };
        }

        if self.consume("{") {
            let mut stmts = Vec::new();
            while !self.consume("}") {
                if self.at_eof() {
                    error_at(self.src, self.peek().pos, "not '}'");
                }
                stmts.push(self.stmt());
            }
            return Node::Block(stmts);
        }

        let node = self.expr();
        self.expect(";");
        node
    }

    fn opt_expr(&mut self, end: &str) -> Option<Node> {
        if self.consume(end) {
            return None;
        }
        let node = self.expr();
        self.expect(end);
        Some(node)
    }

    fn expr(&mut self) -> Node {
        self.assign()
    }

    fn assign(&mut self) -> Node {
        let pos = self.peek().pos;
        let node = self.equality();

        if self.consume("=") {
            if !matches!(node, Node::Var { .. } | Node::Deref(_)) {
                error_at(self.src, pos, "the lhs must be a variable or a * expr");
            }
            return Node::Assign(Box::new(node), Box::new(self.assign()));
        }
        node
    }

    fn equality(&mut self) -> Node {
        let mut node = self.relational();

        loop {
            if self.consume("==") {
                node = bin(BinOp::Eq, node, self.relational());
            } else if self.consume("!=") {
                node = bin(BinOp::Ne, node, self.relational());
            } else {
                return node;
            }
        }
    }

    fn relational(&mut self) -> Node {
        let mut node = self.add();

        loop {
            if self.consume("<") {
                node = bin(BinOp::Lt, node, self.add());
            } else if self.consume("<=") {
                node = bin(BinOp::Le, node, self.add());
            } else if self.consume(">") {
                node = bin(BinOp::Lt, self.add(), node);
            } else if self.consume(">=") {
                node = bin(BinOp::Le, self.add(), node);
            } else {
                return node;
            }
        }
    }

    fn add(&mut self) -> Node {
        let mut node = self.mul();

        loop {
            if self.consume("+") {
                node = bin(BinOp::Add, node, self.mul());
            } else if self.consume("-") {
                node = bin(BinOp::Sub, node, self.mul());
            } else {
                return node;
            }
        }
    }

    fn mul(&mut self) -> Node {
        let mut node = self.unary();

        loop {
            if self.consume("*") {
                node = bin(BinOp::Mul, node, self.unary());
            } else if self.consume("/") {
                node = bin(BinOp::Div, node, self.unary());
            } else {
                return node;
            }
        }
    }

    fn unary(&mut self) -> Node {
        if self.consume("+") {
            return self.unary();
        }
        if self.consume("-") {
            return bin(BinOp::Sub, Node::Num(0), self.unary());
        }
        if self.consume("*") {
            let pos = self.peek().pos;
            let node = self.unary();
            if !matches!(type_of(&node), Type::Ptr(_)) {
                error_at(self.src, pos, "can't deref because this is not pointer")
            }
            return Node::Deref(Box::new(node));
        }
        if self.consume("&") {
            let pos = self.peek().pos;
            let node = self.unary();
            if !matches!(node, Node::Var { .. } | Node::Deref(_)) {
                error_at(self.src, pos, "can't take the address of this")
            }
            return Node::Addr(Box::new(node));
        }
        self.primary()
    }

    fn primary(&mut self) -> Node {
        if self.consume("(") {
            let node = self.expr();
            self.expect(")");
            return node;
        }

        let pos = self.peek().pos;
        if let Some(name) = self.consume_ident() {
            if self.consume("(") {
                let mut args = Vec::new();
                while !self.consume(")") {
                    if !args.is_empty() {
                        self.expect(",");
                    }
                    args.push(self.assign());
                }
                if args.len() > 6 {
                    error_at(self.src, self.peek().pos, "arguments limit is 6")
                }
                return Node::Call(name, args);
            }
            return match self.find_var(&name) {
                Some(var) => Node::Var {
                    offset: var.offset,
                    ty: var.ty.clone(),
                },
                None => error_at(self.src, pos, "not defined variable"),
            };
        }
        Node::Num(self.expect_number())
    }
}
