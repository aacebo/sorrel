use super::*;
#[derive(Debug, Clone)]
pub struct MacroCall {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub path: Path,
    pub delimiter: DelimiterKind,
    pub tokens: crate::TokenStream,
}
