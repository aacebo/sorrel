use super::*;
#[derive(Debug, Clone)]
pub struct LitByteStr {
    pub span: crate::Span,
    pub value: Vec<u8>,
}
