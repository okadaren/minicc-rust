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
    GVar {
        name: String,
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
    pub params: Vec<(i64, Type)>,
    pub body: Vec<Node>,
    pub stack_size: i64,
}

pub struct GlobalVar {
    pub name: String,
    pub ty: Type,
    pub init: Option<Vec<u8>>,
}

pub struct Program {
    pub funcs: Vec<Function>,
    pub globals: Vec<GlobalVar>,
}

fn bin(op: BinOp, l: Node, r: Node) -> Node {
    Node::Binary(op, Box::new(l), Box::new(r))
}

pub struct Parser<'a> {
    src: &'a str,
    toks: Vec<Token>,
    pos: usize,
    locals: Vec<LVar>,
    stack: i64,
    globals: Vec<GlobalVar>,
    str_count: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            src,
            toks: tokenize(src),
            pos: 0,
            locals: Vec::new(),
            stack: 0,
            globals: Vec::new(),
            str_count: 0,
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

    fn is_typename(&self) -> bool {
        matches!(&self.peek().kind, TokenKind::Keyword(k) if k == "int" || k == "char")
    }

    fn parse_type(&mut self) -> Type {
        let mut ty = if self.consume("char") {
            Type::Char
        } else {
            self.expect("int");
            Type::Int
        };
        while self.consume("*") {
            ty = Type::pointer_to(ty);
        }
        ty
    }

    fn declare_var(&mut self, name: String, ty: Type, pos: usize) -> i64 {
        if self.locals.iter().any(|v| v.name == name) {
            error_at(self.src, pos, "same name variable is defined");
        }
        self.stack = (self.stack + ty.size() + 7) / 8 * 8;
        let offset = self.stack;
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
            if self.is_function() {
                funcs.push(self.function());
            } else {
                self.global_var();
            }
        }
        let globals = std::mem::take(&mut self.globals);
        Program { funcs, globals }
    }

    fn is_function(&mut self) -> bool {
        let start = self.pos;
        self.parse_type();
        self.expect_ident("name");
        let result = matches!(&self.peek().kind, TokenKind::Punct(s) if s == "(");
        self.pos = start;
        result
    }

    fn global_var(&mut self) {
        let ty = self.parse_type();
        let pos = self.peek().pos;
        let name = self.expect_ident("variable name");
        let ty = self.array_suffix(ty);
        self.expect(";");
        if self.globals.iter().any(|g| g.name == name) {
            error_at(self.src, pos, "same name global variable defined");
        }
        self.globals.push(GlobalVar {
            name,
            ty,
            init: None,
        });
    }

    fn array_suffix(&mut self, ty: Type) -> Type {
        if self.consume("[") {
            let len = self.expect_number();
            self.expect("]");
            return Type::Array(Box::new(ty), len as usize);
        }
        ty
    }

    fn function(&mut self) -> Function {
        self.locals.clear();
        self.stack = 0;

        let pos = self.peek().pos;
        self.parse_type();
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
            let offset = self.declare_var(param, ty.clone(), pos);
            params.push((offset, ty));
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

        let stack_size = (self.stack + 15) / 16 * 16;
        Function {
            name,
            params,
            body,
            stack_size,
        }
    }

    fn stmt(&mut self) -> Node {
        if self.is_typename() {
            let ty = self.parse_type();
            let pos = self.peek().pos;
            let name = self.expect_ident("variable name");
            let ty = self.array_suffix(ty);
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
            if !matches!(node, Node::Var { .. } | Node::GVar { .. } | Node::Deref(_)) {
                error_at(self.src, pos, "the lhs must be a variable or a * expr");
            }

            if matches!(type_of(&node), Type::Array(..)) {
                error_at(self.src, pos, "can't assign to array");
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
            let pos = self.peek().pos;
            if self.consume("+") {
                let rhs = self.mul();
                node = self.new_add(node, rhs, pos);
            } else if self.consume("-") {
                let rhs = self.mul();
                node = self.new_sub(node, rhs, pos);
            } else {
                return node;
            }
        }
    }

    fn new_add(&self, lhs: Node, rhs: Node, pos: usize) -> Node {
        let lbase = type_of(&lhs).base().cloned();
        let rbase = type_of(&rhs).base().cloned();
        match (lbase, rbase) {
            // 整数 + 整数
            (None, None) => bin(BinOp::Add, lhs, rhs),
            // ポインタ + 整数
            (Some(base), None) => {
                let scaled = bin(BinOp::Mul, rhs, Node::Num(base.size()));
                bin(BinOp::Add, lhs, scaled)
            }
            // 整数 + ポインタ
            (None, Some(base)) => {
                let scaled = bin(BinOp::Mul, lhs, Node::Num(base.size()));
                bin(BinOp::Add, rhs, scaled)
            }
            (Some(_), Some(_)) => error_at(self.src, pos, "can't add two pointers"),
        }
    }

    fn new_sub(&self, lhs: Node, rhs: Node, pos: usize) -> Node {
        let lbase = type_of(&lhs).base().cloned();
        let rbase = type_of(&rhs).base().cloned();
        match (lbase, rbase) {
            // 整数 - 整数
            (None, None) => bin(BinOp::Sub, lhs, rhs),
            // ポインタ - 整数
            (Some(base), None) => {
                let scaled = bin(BinOp::Mul, rhs, Node::Num(base.size()));
                bin(BinOp::Sub, lhs, scaled)
            }
            // ポインタ - ポインタ
            (Some(base), Some(_)) => {
                let diff = bin(BinOp::Sub, lhs, rhs);
                bin(BinOp::Div, diff, Node::Num(base.size()))
            }
            (None, Some(_)) => error_at(self.src, pos, "can't sub pointer from int"),
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
        if self.consume("sizeof") {
            let node = self.unary();
            return Node::Num(type_of(&node).size());
        }
        if self.consume("+") {
            return self.unary();
        }
        if self.consume("-") {
            return bin(BinOp::Sub, Node::Num(0), self.unary());
        }
        if self.consume("*") {
            let pos = self.peek().pos;
            let node = self.unary();
            if type_of(&node).base().is_none() {
                error_at(self.src, pos, "can't deref because this is not pointer")
            }
            return Node::Deref(Box::new(node));
        }
        if self.consume("&") {
            let pos = self.peek().pos;
            let node = self.unary();
            if !matches!(node, Node::Var { .. } | Node::GVar { .. } | Node::Deref(_)) {
                error_at(self.src, pos, "can't take the address of this")
            }
            return Node::Addr(Box::new(node));
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Node {
        let mut node = self.primary();
        loop {
            let pos = self.peek().pos;
            if !self.consume("[") {
                return node;
            }
            let index = self.expr();
            self.expect("]");
            let addr = self.new_add(node, index, pos);
            if type_of(&addr).base().is_none() {
                error_at(self.src, pos, "this is not pointer or array, can't deref");
            }
            node = Node::Deref(Box::new(addr))
        }
    }

    fn primary(&mut self) -> Node {
        if self.consume("(") {
            let node = self.expr();
            self.expect(")");
            return node;
        }

        if let TokenKind::Str(bytes) = &self.peek().kind {
            let mut bytes = bytes.clone();
            self.pos += 1;
            bytes.push(0);
            let name = format!(".L.str.{}", self.str_count);
            self.str_count += 1;
            let ty = Type::Array(Box::new(Type::Char), bytes.len());
            self.globals.push(GlobalVar {
                name: name.clone(),
                ty: ty.clone(),
                init: Some(bytes),
            });
            return Node::GVar { name, ty };
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
            if let Some(var) = self.find_var(&name) {
                return Node::Var {
                    offset: var.offset,
                    ty: var.ty.clone(),
                };
            }
            if let Some(g) = self.globals.iter().find(|g| g.name == name) {
                return Node::GVar {
                    name,
                    ty: g.ty.clone(),
                };
            }
            error_at(self.src, pos, "not defined variable");
        }
        Node::Num(self.expect_number())
    }
}
