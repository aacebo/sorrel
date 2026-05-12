use super::*;
#[derive(Debug, Clone)]
pub struct ExprIf {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub cond: Box<Expr>,
    pub then_branch: Block,
    pub else_branch: Option<Box<Expr>>,
}
