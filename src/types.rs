use crate::parse::{BinOp, Node};

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Ptr(Box<Type>),
}

impl Type {
    pub fn pointer_to(base: Type) -> Type {
        Type::Ptr(Box::new(base))
    }

    pub fn size(&self) -> i64 {
        match self {
            Type::Int => 4,
            Type::Ptr(_) => 8,
        }
    }
}

pub fn type_of(node: &Node) -> Type {
    match node {
        Node::Var { ty, .. } => ty.clone(),
        Node::Addr(e) => Type::pointer_to(type_of(e)),
        Node::Deref(e) => match type_of(e) {
            Type::Ptr(base) => *base,
            Type::Int => unreachable!(),
        },
        Node::Assign(lhs, _) => type_of(lhs),
        Node::Binary(BinOp::Add | BinOp::Sub, lhs, _) => match type_of(lhs) {
            ty @ Type::Ptr(_) => ty,
            Type::Int => Type::Int,
        },
        _ => Type::Int,
    }
}
