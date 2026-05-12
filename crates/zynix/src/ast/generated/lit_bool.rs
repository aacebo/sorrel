use super::*;
#[derive(Debug, Clone)]
pub struct LitBool {
    pub span: crate::Span,
    pub value: bool,
}
