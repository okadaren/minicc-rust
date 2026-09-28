use crate::error::{Result, error_at};
use crate::tokenize::{Keyword, Punct, Token, TokenKind, tokenize};
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

pub struct Param {
    pub offset: i64,
    pub ty: Type,
}

pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
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

fn new_add(lhs: Node, rhs: Node, pos: usize) -> Result<Node> {
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

fn new_sub(lhs: Node, rhs: Node, pos: usize) -> Result<Node> {
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

    fn peek(&self) -> &Token {
        &self.toks[self.pos]
    }

    // 次が記号なら読み進めてtrueを返す
    fn consume(&mut self, kind: impl Into<TokenKind>) -> bool {
        if self.peek().kind == kind.into() {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    // 次が記号でなければエラー
    fn expect(&mut self, op: Punct) -> Result<()> {
        if !self.consume(op) {
            return error_at(self.peek().pos, format!("expected '{}'", op));
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
            None => error_at(pos, format!("expected {}", what)),
        }
    }

    fn is_typename(&self) -> bool {
        matches!(
            &self.peek().kind,
            TokenKind::Keyword(Keyword::Int | Keyword::Char)
        )
    }

    fn parse_type(&mut self) -> Result<Type> {
        let mut ty = match self.peek().kind {
            TokenKind::Keyword(Keyword::Char) => Type::Char,
            TokenKind::Keyword(Keyword::Int) => Type::Int,
            _ => return error_at(self.peek().pos, "expected a type name"),
        };
        self.pos += 1;
        while self.consume(Punct::Star) {
            ty = Type::pointer_to(ty);
        }
        Ok(ty)
    }

    fn declare_var(&mut self, name: String, ty: Type, pos: usize) -> Result<i64> {
        if self.locals.iter().any(|v| v.name == name) {
            return error_at(pos, format!("redefinition of '{}'", name));
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
        let result = matches!(&self.peek().kind, TokenKind::Punct(Punct::LParen));
        self.pos = start;
        Ok(result)
    }

    fn global_var(&mut self) -> Result<()> {
        let ty = self.parse_type()?;
        let pos = self.peek().pos;
        let name = self.expect_ident("variable name")?;
        let ty = self.array_suffix(ty)?;
        self.expect(Punct::Semi)?;
        if self.globals.iter().any(|g| g.name == name) {
            return error_at(pos, format!("redefinition of '{}'", name));
        }
        self.globals.push(GlobalVar {
            name,
            ty,
            init: None,
        });
        Ok(())
    }

    fn array_suffix(&mut self, ty: Type) -> Result<Type> {
        if self.consume(Punct::LBracket) {
            let len = self.expect_number()?;
            self.expect(Punct::RBracket)?;
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

        self.expect(Punct::LParen)?;
        let mut params = Vec::new();
        while !self.consume(Punct::RParen) {
            if !params.is_empty() {
                self.expect(Punct::Comma)?;
            }
            let ty = self.parse_type()?;
            let pos = self.peek().pos;
            let param = self.expect_ident("argument name")?;
            let offset = self.declare_var(param, ty.clone(), pos)?;
            params.push(Param { offset, ty });
        }
        if params.len() > 6 {
            return error_at(pos, "too many parameters (max 6)");
        }

        self.expect(Punct::LBrace)?;
        let mut body = Vec::new();
        while !self.consume(Punct::RBrace) {
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
            self.expect(Punct::Semi)?;
            return Ok(Node::Block(Vec::new()));
        }

        if self.consume(Keyword::Return) {
            let node = self.expr()?;
            self.expect(Punct::Semi)?;
            return Ok(Node::Return(Box::new(node)));
        }

        if self.consume(Keyword::If) {
            self.expect(Punct::LParen)?;
            let cond = Box::new(self.expr()?);
            self.expect(Punct::RParen)?;
            let then = Box::new(self.stmt()?);
            let els = if self.consume(Keyword::Else) {
                Some(Box::new(self.stmt()?))
            } else {
                None
            };
            return Ok(Node::If { cond, then, els });
        }

        if self.consume(Keyword::While) {
            self.expect(Punct::LParen)?;
            let cond = Some(Box::new(self.expr()?));
            self.expect(Punct::RParen)?;
            let body = Box::new(self.stmt()?);
            return Ok(Node::For {
                init: None,
                cond,
                inc: None,
                body,
            });
        }

        if self.consume(Keyword::For) {
            self.expect(Punct::LParen)?;
            let init = self.opt_expr(Punct::Semi)?.map(Box::new);
            let cond = self.opt_expr(Punct::Semi)?.map(Box::new);
            let inc = self.opt_expr(Punct::RParen)?.map(Box::new);
            let body = Box::new(self.stmt()?);
            return Ok(Node::For {
                init,
                cond,
                inc,
                body,
            });
        }

        if self.consume(Punct::LBrace) {
            let mut stmts = Vec::new();
            while !self.consume(Punct::RBrace) {
                if self.at_eof() {
                    return error_at(self.peek().pos, "expected '}' before end of file");
                }
                stmts.push(self.stmt()?);
            }
            return Ok(Node::Block(stmts));
        }

        let node = self.expr()?;
        self.expect(Punct::Semi)?;
        Ok(node)
    }

    fn opt_expr(&mut self, end: Punct) -> Result<Option<Node>> {
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

        if self.consume(Punct::Assign) {
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
            if self.consume(Punct::Eq) {
                node = bin(BinOp::Eq, node, self.relational()?);
            } else if self.consume(Punct::Ne) {
                node = bin(BinOp::Ne, node, self.relational()?);
            } else {
                return Ok(node);
            }
        }
    }

    fn relational(&mut self) -> Result<Node> {
        let mut node = self.add()?;

        loop {
            if self.consume(Punct::Lt) {
                node = bin(BinOp::Lt, node, self.add()?);
            } else if self.consume(Punct::Le) {
                node = bin(BinOp::Le, node, self.add()?);
            } else if self.consume(Punct::Gt) {
                node = bin(BinOp::Lt, self.add()?, node);
            } else if self.consume(Punct::Ge) {
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
            if self.consume(Punct::Plus) {
                let rhs = self.mul()?;
                node = new_add(node, rhs, pos)?;
            } else if self.consume(Punct::Minus) {
                let rhs = self.mul()?;
                node = new_sub(node, rhs, pos)?;
            } else {
                return Ok(node);
            }
        }
    }

    fn mul(&mut self) -> Result<Node> {
        let mut node = self.unary()?;

        loop {
            if self.consume(Punct::Star) {
                node = bin(BinOp::Mul, node, self.unary()?);
            } else if self.consume(Punct::Slash) {
                node = bin(BinOp::Div, node, self.unary()?);
            } else {
                return Ok(node);
            }
        }
    }

    fn unary(&mut self) -> Result<Node> {
        if self.consume(Keyword::Sizeof) {
            let node = self.unary()?;
            return Ok(Node::Num(type_of(&node).size()));
        }
        if self.consume(Punct::Plus) {
            return self.unary();
        }
        if self.consume(Punct::Minus) {
            return Ok(bin(BinOp::Sub, Node::Num(0), self.unary()?));
        }
        if self.consume(Punct::Star) {
            let pos = self.peek().pos;
            let node = self.unary()?;
            if type_of(&node).base().is_none() {
                return error_at(pos, "cannot dereference a non-pointer value");
            }
            return Ok(Node::Deref(Box::new(node)));
        }
        if self.consume(Punct::Amp) {
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
            if !self.consume(Punct::LBracket) {
                return Ok(node);
            }
            let index = self.expr()?;
            self.expect(Punct::RBracket)?;
            let addr = new_add(node, index, pos)?;
            if type_of(&addr).base().is_none() {
                return error_at(pos, "subscripted value is not an array or pointer");
            }
            node = Node::Deref(Box::new(addr))
        }
    }

    fn primary(&mut self) -> Result<Node> {
        if self.consume(Punct::LParen) {
            let node = self.expr()?;
            self.expect(Punct::RParen)?;
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
            if self.consume(Punct::LParen) {
                let mut args = Vec::new();
                while !self.consume(Punct::RParen) {
                    if !args.is_empty() {
                        self.expect(Punct::Comma)?;
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
            return error_at(pos, format!("undefined variable '{}'", name));
        }
        Ok(Node::Num(self.expect_number()?))
    }
}

#[cfg(test)]
mod tests {
    use crate::compile;

    // ソース中の `^` の位置で msg のエラーになることを確認する
    // `^` は取り除いてからコンパイルする
    fn assert_error(marked: &str, msg: &str) {
        let pos = marked.find('^').expect("no ^ marker");
        let src = marked.replacen('^', "", 1);
        let err = compile(&src)
            .err()
            .unwrap_or_else(|| panic!("expected error '{}' but compiled: {}", msg, src));
        assert_eq!((err.pos, err.msg.as_str()), (pos, msg), "src: {}", src);
    }

    #[test]
    fn valid_program_compiles() {
        let src = "int g; int add(int a, int b) { return a + b; }
                   int main() { int *p; int a[3]; p = a; *(p + 1) = 2;
                                return add(a[1], sizeof(a)); }";
        assert!(compile(src).is_ok());
    }

    // 期待したトークンがない

    #[test]
    fn expected_semicolon() {
        assert_error("int main() { return 1 ^}", "expected ';'");
    }

    #[test]
    fn expected_close_paren() {
        assert_error("int main() { return (1 + 2^; }", "expected ')'");
    }

    #[test]
    fn expected_close_bracket() {
        assert_error("int a[3^;", "expected ']'");
    }

    #[test]
    fn expected_number() {
        assert_error("int main() { return 1 + ^; }", "expected a number");
    }

    #[test]
    fn expected_array_length() {
        assert_error("int a[^x];", "expected a number");
    }

    #[test]
    fn expected_variable_name() {
        assert_error("int main() { int ^; }", "expected variable name");
    }

    #[test]
    fn expected_type_name() {
        assert_error("^1;", "expected a type name");
    }

    #[test]
    fn expected_close_brace_in_function() {
        assert_error("int main() { return 1;^", "expected '}' before end of file");
    }

    #[test]
    fn expected_close_brace_in_block() {
        assert_error(
            "int main() { { return 1;^",
            "expected '}' before end of file",
        );
    }

    // 変数

    #[test]
    fn redefinition_of_local() {
        assert_error("int main() { int a; int ^a; }", "redefinition of 'a'");
    }

    #[test]
    fn redefinition_of_param() {
        assert_error("int f(int a, int ^a) { return 0; }", "redefinition of 'a'");
    }

    #[test]
    fn redefinition_of_global() {
        assert_error("int x; int ^x;", "redefinition of 'x'");
    }

    #[test]
    fn undefined_variable() {
        assert_error("int main() { return ^y; }", "undefined variable 'y'");
    }

    // 関数

    #[test]
    fn too_many_parameters() {
        assert_error(
            "^int f(int a, int b, int c, int d, int e, int f, int g) { return 0; }",
            "too many parameters (max 6)",
        );
    }

    #[test]
    fn too_many_arguments() {
        assert_error(
            "int main() { return ^foo(1, 2, 3, 4, 5, 6, 7); }",
            "too many arguments (max 6)",
        );
    }

    // 代入

    #[test]
    fn assign_to_non_lvalue() {
        assert_error("int main() { ^1 = 2; }", "expression is not assignable");
    }

    #[test]
    fn assign_to_array() {
        assert_error(
            "int main() { int a[2]; int b[2]; ^a = b; }",
            "array type is not assignable",
        );
    }

    // ポインタ演算

    #[test]
    fn add_two_pointers() {
        assert_error(
            "int main() { int *p; int *q; return p ^+ q; }",
            "invalid operands to '+' (pointer + pointer)",
        );
    }

    #[test]
    fn sub_pointer_from_integer() {
        assert_error(
            "int main() { int *p; return 1 ^- p; }",
            "invalid operands to '-' (integer - pointer)",
        );
    }

    #[test]
    fn deref_non_pointer() {
        assert_error(
            "int main() { return *^1; }",
            "cannot dereference a non-pointer value",
        );
    }

    #[test]
    fn address_of_non_lvalue() {
        assert_error(
            "int main() { return &^1; }",
            "cannot take the address of this expression",
        );
    }

    #[test]
    fn subscript_non_pointer() {
        assert_error(
            "int main() { return 1^[2]; }",
            "subscripted value is not an array or pointer",
        );
    }

    // トークナイズのエラーも compile から返ってくる

    #[test]
    fn tokenize_error_is_propagated() {
        assert_error("int main() { return ^@; }", "unexpected character '@'");
    }
}
