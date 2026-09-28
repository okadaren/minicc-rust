use crate::error::{Result, error_at};
use crate::tokenize::{Token, TokenKind, tokenize};
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

pub struct Parser {
    toks: Vec<Token>,
    pos: usize,
    locals: Vec<LVar>,
    stack: i64,
    globals: Vec<GlobalVar>,
    str_count: usize,
}

impl Parser {
    pub fn new(src: &str) -> Result<Self> {
        Ok(Parser {
            toks: tokenize(src)?,
            pos: 0,
            locals: Vec::new(),
            stack: 0,
            globals: Vec::new(),
            str_count: 0,
        })
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
    fn expect(&mut self, op: &str) -> Result<()> {
        if !self.consume(op) {
            return error_at(self.peek().pos, &format!("expected '{}'", op));
        }
        Ok(())
    }

    // 次が整数なら読み進めてその値を返す。そうでなければエラー
    fn expect_number(&mut self) -> Result<i64> {
        match self.peek().kind {
            TokenKind::Num(n) => {
                self.pos += 1;
                Ok(n)
            }
            _ => error_at(self.peek().pos, "expected a number"),
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

    fn expect_ident(&mut self, what: &str) -> Result<String> {
        let pos = self.peek().pos;
        match self.consume_ident() {
            Some(name) => Ok(name),
            None => error_at(pos, &format!("expected {}", what)),
        }
    }

    fn is_typename(&self) -> bool {
        matches!(&self.peek().kind, TokenKind::Keyword(k) if k == "int" || k == "char")
    }

    fn parse_type(&mut self) -> Result<Type> {
        let mut ty = if self.consume("char") {
            Type::Char
        } else if self.consume("int") {
            Type::Int
        } else {
            return error_at(self.peek().pos, "expected a type name");
        };
        while self.consume("*") {
            ty = Type::pointer_to(ty);
        }
        Ok(ty)
    }

    fn declare_var(&mut self, name: String, ty: Type, pos: usize) -> Result<i64> {
        if self.locals.iter().any(|v| v.name == name) {
            return error_at(pos, &format!("redefinition of '{}'", name));
        }
        self.stack = (self.stack + ty.size() + 7) / 8 * 8;
        let offset = self.stack;
        self.locals.push(LVar { name, offset, ty });
        Ok(offset)
    }

    fn find_var(&self, name: &str) -> Option<&LVar> {
        self.locals.iter().find(|v| v.name == name)
    }

    fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    pub fn program(&mut self) -> Result<Program> {
        let mut funcs = Vec::new();
        while !self.at_eof() {
            if self.is_function()? {
                funcs.push(self.function()?);
            } else {
                self.global_var()?;
            }
        }
        let globals = std::mem::take(&mut self.globals);
        Ok(Program { funcs, globals })
    }

    fn is_function(&mut self) -> Result<bool> {
        let start = self.pos;
        self.parse_type()?;
        self.expect_ident("name")?;
        let result = matches!(&self.peek().kind, TokenKind::Punct(s) if s == "(");
        self.pos = start;
        Ok(result)
    }

    fn global_var(&mut self) -> Result<()> {
        let ty = self.parse_type()?;
        let pos = self.peek().pos;
        let name = self.expect_ident("variable name")?;
        let ty = self.array_suffix(ty)?;
        self.expect(";")?;
        if self.globals.iter().any(|g| g.name == name) {
            return error_at(pos, &format!("redefinition of '{}'", name));
        }
        self.globals.push(GlobalVar {
            name,
            ty,
            init: None,
        });
        Ok(())
    }

    fn array_suffix(&mut self, ty: Type) -> Result<Type> {
        if self.consume("[") {
            let len = self.expect_number()?;
            self.expect("]")?;
            return Ok(Type::Array(Box::new(ty), len as usize));
        }
        Ok(ty)
    }

    fn function(&mut self) -> Result<Function> {
        self.locals.clear();
        self.stack = 0;

        let pos = self.peek().pos;
        self.parse_type()?;
        let name = self.expect_ident("function name")?;

        self.expect("(")?;
        let mut params = Vec::new();
        while !self.consume(")") {
            if !params.is_empty() {
                self.expect(",")?;
            }
            let ty = self.parse_type()?;
            let pos = self.peek().pos;
            let param = self.expect_ident("argument name")?;
            let offset = self.declare_var(param, ty.clone(), pos)?;
            params.push((offset, ty));
        }
        if params.len() > 6 {
            return error_at(pos, "too many parameters (max 6)");
        }

        self.expect("{")?;
        let mut body = Vec::new();
        while !self.consume("}") {
            if self.at_eof() {
                return error_at(self.peek().pos, "expected '}' before end of file");
            }
            body.push(self.stmt()?);
        }

        let stack_size = (self.stack + 15) / 16 * 16;
        Ok(Function {
            name,
            params,
            body,
            stack_size,
        })
    }

    fn stmt(&mut self) -> Result<Node> {
        if self.is_typename() {
            let ty = self.parse_type()?;
            let pos = self.peek().pos;
            let name = self.expect_ident("variable name")?;
            let ty = self.array_suffix(ty)?;
            self.declare_var(name, ty, pos)?;
            self.expect(";")?;
            return Ok(Node::Block(Vec::new()));
        }

        if self.consume("return") {
            let node = self.expr()?;
            self.expect(";")?;
            return Ok(Node::Return(Box::new(node)));
        }

        if self.consume("if") {
            self.expect("(")?;
            let cond = Box::new(self.expr()?);
            self.expect(")")?;
            let then = Box::new(self.stmt()?);
            let els = if self.consume("else") {
                Some(Box::new(self.stmt()?))
            } else {
                None
            };
            return Ok(Node::If { cond, then, els });
        }

        if self.consume("while") {
            self.expect("(")?;
            let cond = Some(Box::new(self.expr()?));
            self.expect(")")?;
            let body = Box::new(self.stmt()?);
            return Ok(Node::For {
                init: None,
                cond,
                inc: None,
                body,
            });
        }

        if self.consume("for") {
            self.expect("(")?;
            let init = self.opt_expr(";")?.map(Box::new);
            let cond = self.opt_expr(";")?.map(Box::new);
            let inc = self.opt_expr(")")?.map(Box::new);
            let body = Box::new(self.stmt()?);
            return Ok(Node::For {
                init,
                cond,
                inc,
                body,
            });
        }

        if self.consume("{") {
            let mut stmts = Vec::new();
            while !self.consume("}") {
                if self.at_eof() {
                    return error_at(self.peek().pos, "expected '}' before end of file");
                }
                stmts.push(self.stmt()?);
            }
            return Ok(Node::Block(stmts));
        }

        let node = self.expr()?;
        self.expect(";")?;
        Ok(node)
    }

    fn opt_expr(&mut self, end: &str) -> Result<Option<Node>> {
        if self.consume(end) {
            return Ok(None);
        }
        let node = self.expr()?;
        self.expect(end)?;
        Ok(Some(node))
    }

    fn expr(&mut self) -> Result<Node> {
        self.assign()
    }

    fn assign(&mut self) -> Result<Node> {
        let pos = self.peek().pos;
        let node = self.equality()?;

        if self.consume("=") {
            if !matches!(node, Node::Var { .. } | Node::GVar { .. } | Node::Deref(_)) {
                return error_at(pos, "expression is not assignable");
            }

            if matches!(type_of(&node), Type::Array(..)) {
                return error_at(pos, "array type is not assignable");
            }
            return Ok(Node::Assign(Box::new(node), Box::new(self.assign()?)));
        }
        Ok(node)
    }

    fn equality(&mut self) -> Result<Node> {
        let mut node = self.relational()?;

        loop {
            if self.consume("==") {
                node = bin(BinOp::Eq, node, self.relational()?);
            } else if self.consume("!=") {
                node = bin(BinOp::Ne, node, self.relational()?);
            } else {
                return Ok(node);
            }
        }
    }

    fn relational(&mut self) -> Result<Node> {
        let mut node = self.add()?;

        loop {
            if self.consume("<") {
                node = bin(BinOp::Lt, node, self.add()?);
            } else if self.consume("<=") {
                node = bin(BinOp::Le, node, self.add()?);
            } else if self.consume(">") {
                node = bin(BinOp::Lt, self.add()?, node);
            } else if self.consume(">=") {
                node = bin(BinOp::Le, self.add()?, node);
            } else {
                return Ok(node);
            }
        }
    }

    fn add(&mut self) -> Result<Node> {
        let mut node = self.mul()?;

        loop {
            let pos = self.peek().pos;
            if self.consume("+") {
                let rhs = self.mul()?;
                node = self.new_add(node, rhs, pos)?;
            } else if self.consume("-") {
                let rhs = self.mul()?;
                node = self.new_sub(node, rhs, pos)?;
            } else {
                return Ok(node);
            }
        }
    }

    fn new_add(&self, lhs: Node, rhs: Node, pos: usize) -> Result<Node> {
        let lbase = type_of(&lhs).base().cloned();
        let rbase = type_of(&rhs).base().cloned();
        match (lbase, rbase) {
            // 整数 + 整数
            (None, None) => Ok(bin(BinOp::Add, lhs, rhs)),
            // ポインタ + 整数
            (Some(base), None) => {
                let scaled = bin(BinOp::Mul, rhs, Node::Num(base.size()));
                Ok(bin(BinOp::Add, lhs, scaled))
            }
            // 整数 + ポインタ
            (None, Some(base)) => {
                let scaled = bin(BinOp::Mul, lhs, Node::Num(base.size()));
                Ok(bin(BinOp::Add, rhs, scaled))
            }
            (Some(_), Some(_)) => error_at(pos, "invalid operands to '+' (pointer + pointer)"),
        }
    }

    fn new_sub(&self, lhs: Node, rhs: Node, pos: usize) -> Result<Node> {
        let lbase = type_of(&lhs).base().cloned();
        let rbase = type_of(&rhs).base().cloned();
        match (lbase, rbase) {
            // 整数 - 整数
            (None, None) => Ok(bin(BinOp::Sub, lhs, rhs)),
            // ポインタ - 整数
            (Some(base), None) => {
                let scaled = bin(BinOp::Mul, rhs, Node::Num(base.size()));
                Ok(bin(BinOp::Sub, lhs, scaled))
            }
            // ポインタ - ポインタ
            (Some(base), Some(_)) => {
                let diff = bin(BinOp::Sub, lhs, rhs);
                Ok(bin(BinOp::Div, diff, Node::Num(base.size())))
            }
            (None, Some(_)) => error_at(pos, "invalid operands to '-' (integer - pointer)"),
        }
    }

    fn mul(&mut self) -> Result<Node> {
        let mut node = self.unary()?;

        loop {
            if self.consume("*") {
                node = bin(BinOp::Mul, node, self.unary()?);
            } else if self.consume("/") {
                node = bin(BinOp::Div, node, self.unary()?);
            } else {
                return Ok(node);
            }
        }
    }

    fn unary(&mut self) -> Result<Node> {
        if self.consume("sizeof") {
            let node = self.unary()?;
            return Ok(Node::Num(type_of(&node).size()));
        }
        if self.consume("+") {
            return self.unary();
        }
        if self.consume("-") {
            return Ok(bin(BinOp::Sub, Node::Num(0), self.unary()?));
        }
        if self.consume("*") {
            let pos = self.peek().pos;
            let node = self.unary()?;
            if type_of(&node).base().is_none() {
                return error_at(pos, "cannot dereference a non-pointer value");
            }
            return Ok(Node::Deref(Box::new(node)));
        }
        if self.consume("&") {
            let pos = self.peek().pos;
            let node = self.unary()?;
            if !matches!(node, Node::Var { .. } | Node::GVar { .. } | Node::Deref(_)) {
                return error_at(pos, "cannot take the address of this expression");
            }
            return Ok(Node::Addr(Box::new(node)));
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Result<Node> {
        let mut node = self.primary()?;
        loop {
            let pos = self.peek().pos;
            if !self.consume("[") {
                return Ok(node);
            }
            let index = self.expr()?;
            self.expect("]")?;
            let addr = self.new_add(node, index, pos)?;
            if type_of(&addr).base().is_none() {
                return error_at(pos, "subscripted value is not an array or pointer");
            }
            node = Node::Deref(Box::new(addr))
        }
    }

    fn primary(&mut self) -> Result<Node> {
        if self.consume("(") {
            let node = self.expr()?;
            self.expect(")")?;
            return Ok(node);
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
            return Ok(Node::GVar { name, ty });
        }

        let pos = self.peek().pos;
        if let Some(name) = self.consume_ident() {
            if self.consume("(") {
                let mut args = Vec::new();
                while !self.consume(")") {
                    if !args.is_empty() {
                        self.expect(",")?;
                    }
                    args.push(self.assign()?);
                }
                if args.len() > 6 {
                    return error_at(pos, "too many arguments (max 6)");
                }
                return Ok(Node::Call(name, args));
            }
            if let Some(var) = self.find_var(&name) {
                return Ok(Node::Var {
                    offset: var.offset,
                    ty: var.ty.clone(),
                });
            }
            if let Some(g) = self.globals.iter().find(|g| g.name == name) {
                return Ok(Node::GVar {
                    name,
                    ty: g.ty.clone(),
                });
            }
            return error_at(pos, &format!("undefined variable '{}'", name));
        }
        Ok(Node::Num(self.expect_number()?))
    }
}
