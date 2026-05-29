#[allow(unused)]
use super::*;
#[doc = "A binary operation expression (e.g. `a + b`, `x == y`)."]
#[derive(Debug, Clone)]
pub struct ExprBinary {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}
