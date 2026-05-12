use super::*;
#[derive(Debug, Clone)]
pub struct LitFloat {
    pub span: crate::Span,
    pub digits: String,
    pub suffix: Option<Ident>,
}
