#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprBreak {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub expr: Option<Box<Expr>>,
}
