#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatLit {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub expr: Expr,
}
