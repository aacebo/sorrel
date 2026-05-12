use super::*;
#[derive(Debug, Clone)]
pub struct ExprContinue {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
}
