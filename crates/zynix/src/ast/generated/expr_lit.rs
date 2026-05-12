use super::*;
#[derive(Debug, Clone)]
pub struct ExprLit {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub lit: Lit,
}
