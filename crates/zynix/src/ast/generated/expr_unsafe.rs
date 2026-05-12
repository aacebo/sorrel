use super::*;
#[derive(Debug, Clone)]
pub struct ExprUnsafe {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub block: Block,
}
