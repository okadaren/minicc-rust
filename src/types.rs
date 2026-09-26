use crate::parse::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Ptr(Box<Type>),
}

impl Type {
    pub fn pointer_to(base: Type) -> Type {
        Type::Ptr(Box::new(base))
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
        _ => Type::Int,
    }
}
