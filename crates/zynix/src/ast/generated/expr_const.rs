use super::*;
#[derive(Debug, Clone)]
pub struct ExprConst {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub block: Block,
}
