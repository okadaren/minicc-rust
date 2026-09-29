use crate::parse::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Char,
    Int,
    Ptr(Box<Type>),
    Array(Box<Type>, usize),
}

impl Type {
    pub fn pointer_to(base: Type) -> Type {
        Type::Ptr(Box::new(base))
    }

    pub fn size(&self) -> i64 {
        match self {
            Type::Char => 1,
            Type::Int => 4,
            Type::Ptr(_) => 8,
            Type::Array(base, len) => base.size() * *len as i64,
        }
    }

    pub fn base(&self) -> Option<&Type> {
        match self {
            Type::Ptr(base) | Type::Array(base, _) => Some(base),
            Type::Char | Type::Int => None,
        }
    }
}

pub fn type_of(node: &Expr) -> Type {
    node.ty.clone()
}
