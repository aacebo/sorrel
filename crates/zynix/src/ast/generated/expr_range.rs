use super::*;
#[derive(Debug, Clone)]
pub struct ExprRange {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub start: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<Expr>>,
}
