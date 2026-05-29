#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeReference {
    pub span: crate::Span,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}
