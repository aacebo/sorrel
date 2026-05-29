#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprAssign {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
