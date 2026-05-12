use super::*;
#[derive(Debug, Clone)]
pub struct LitInt {
    pub span: crate::Span,
    pub digits: String,
    pub suffix: Option<Ident>,
}
