use crate::tokenize::{Token, TokenKind, error_at, tokenize};

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
    Var(i64),
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
}

struct LVar {
    name: String,
    offset: i64,
}

pub struct Program {
    pub body: Vec<Node>,
    pub stack_size: i64,
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

    fn var_offset(&mut self, name: &str) -> i64 {
        if let Some(var) = self.locals.iter().find(|v| v.name == name) {
            return var.offset;
        }
        let offset = (self.locals.len() as i64 + 1) * 8;
        self.locals.push(LVar {
            name: name.to_string(),
            offset,
        });
        offset
    }

    fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    pub fn program(&mut self) -> Program {
        let mut body = Vec::new();
        while !self.at_eof() {
            body.push(self.stmt());
        }
        let stack_size = (self.locals.len() as i64 * 8 + 15) / 16 * 16;
        Program { body, stack_size }
    }

    fn stmt(&mut self) -> Node {
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
            if !matches!(node, Node::Var(_)) {
                error_at(self.src, pos, "lhs is not variable");
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
                node = bin(BinOp::Mul, node, self.unary())
            } else if self.consume("/") {
                node = bin(BinOp::Div, node, self.unary())
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
        self.primary()
    }

    fn primary(&mut self) -> Node {
        if self.consume("(") {
            let node = self.expr();
            self.expect(")");
            return node;
        }
        if let Some(name) = self.consume_ident() {
            return Node::Var(self.var_offset(&name));
        }
        Node::Num(self.expect_number())
    }
}
