#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprAwait {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<Expr>,
}
