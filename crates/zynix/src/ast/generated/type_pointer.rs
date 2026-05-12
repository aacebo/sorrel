use super::*;
#[derive(Debug, Clone)]
pub struct TypePointer {
    pub span: crate::Span,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}
