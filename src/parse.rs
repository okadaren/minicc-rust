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
}

fn bin(op: BinOp, l: Node, r: Node) -> Node {
    Node::Binary(op, Box::new(l), Box::new(r))
}

pub struct Parser<'a> {
    src: &'a str,
    toks: Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            src,
            toks: tokenize(src),
            pos: 0,
        }
    }

    pub fn peek(&self) -> &Token {
        &self.toks[self.pos]
    }

    // 次が記号なら読み進めてtrueを返す
    fn consume(&mut self, op: &str) -> bool {
        if matches!(&self.peek().kind, TokenKind::Punct(s) if s == op) {
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

    fn consume_ident(&mut self) -> Option<char> {
        match self.peek().kind {
            TokenKind::Ident(c) => {
                self.pos += 1;
                Some(c)
            }
            _ => None,
        }
    }

    fn at_eof(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    pub fn program(&mut self) -> Vec<Node> {
        let mut stmts = Vec::new();
        while !self.at_eof() {
            stmts.push(self.stmt());
        }
        stmts
    }

    fn stmt(&mut self) -> Node {
        let node = self.expr();
        self.expect(";");
        node
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
        if let Some(c) = self.consume_ident() {
            let offset = (c as i64 - 'a' as i64 + 1) * 8;
            return Node::Var(offset);
        }
        Node::Num(self.expect_number())
    }
}
