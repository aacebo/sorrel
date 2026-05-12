use super::*;
#[derive(Debug, Clone)]
pub struct ExprAsync {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub capture: bool,
    pub block: Block,
}
