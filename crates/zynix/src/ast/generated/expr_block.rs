use super::*;
#[derive(Debug, Clone)]
pub struct ExprBlock {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub block: Block,
}
