#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprIndex {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<Expr>,
    pub index: Box<Expr>,
}
