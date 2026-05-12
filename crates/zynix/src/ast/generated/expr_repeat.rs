use super::*;
#[derive(Debug, Clone)]
pub struct ExprRepeat {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elem: Box<Expr>,
    pub len: Box<Expr>,
}
