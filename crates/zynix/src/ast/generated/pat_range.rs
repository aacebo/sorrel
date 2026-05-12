use super::*;
#[derive(Debug, Clone)]
pub struct PatRange {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub start: Option<Expr>,
    pub limits: RangeLimits,
    pub end: Option<Expr>,
}
