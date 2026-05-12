use super::*;
#[derive(Debug, Clone)]
pub struct ExprYield {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub expr: Option<Box<Expr>>,
}
