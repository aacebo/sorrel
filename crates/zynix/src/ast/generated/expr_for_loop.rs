use super::*;
#[derive(Debug, Clone)]
pub struct ExprForLoop {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub pat: Box<Pattern>,
    pub expr: Box<Expr>,
    pub body: Block,
}
