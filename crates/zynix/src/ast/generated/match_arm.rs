#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Expr,
}
