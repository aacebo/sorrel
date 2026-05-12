#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprType {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
    pub ty: Box<Type>,
}
