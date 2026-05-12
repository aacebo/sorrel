use super::*;
#[derive(Debug, Clone)]
pub struct ExprReference {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub mutability: Mutability,
    pub expr: Box<Expr>,
}
