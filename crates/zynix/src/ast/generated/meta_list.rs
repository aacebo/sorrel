#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct MetaList {
    pub span: crate::Span,
    pub path: Path,
    pub delimiter: DelimiterKind,
    pub tokens: crate::TokenStream,
}
