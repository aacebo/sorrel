use super::*;
#[derive(Debug, Clone)]
pub struct LitCStr {
    pub span: crate::Span,
    pub value: Vec<u8>,
}
