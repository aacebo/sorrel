#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprWhile {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub cond: Box<Expr>,
    pub body: Block,
}
