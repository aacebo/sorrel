#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprField {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<Expr>,
    pub member: Member,
}
