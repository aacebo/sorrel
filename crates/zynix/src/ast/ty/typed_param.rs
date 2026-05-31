#[allow(unused)]
use crate::ast::*;

#[derive(Debug, Clone)]
pub struct TypedParam {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub ty: Type,
}
