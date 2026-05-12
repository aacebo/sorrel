#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatReference {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub mutability: Mutability,
    pub pat: Box<Pattern>,
}
