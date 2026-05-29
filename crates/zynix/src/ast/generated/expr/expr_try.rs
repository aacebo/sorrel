#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprTry {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
}
