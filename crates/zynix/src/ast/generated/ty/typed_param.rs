#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypedParam {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub ty: Type,
}
