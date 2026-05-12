use super::*;
#[derive(Debug, Clone)]
pub struct DocString {
    pub span: crate::Span,
    pub value: String,
    pub style: AttrStyle,
}
