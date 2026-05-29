#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LocalInit {
    pub span: crate::Span,
    pub expr: Expr,
    pub diverge: Option<Box<Expr>>,
}
