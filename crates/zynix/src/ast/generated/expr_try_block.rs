use super::*;
#[derive(Debug, Clone)]
pub struct ExprTryBlock {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub block: Block,
}
