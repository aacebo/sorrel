use super::*;
#[derive(Debug, Clone)]
pub struct MetaNameValue {
    pub span: crate::Span,
    pub path: Path,
    pub value: Expr,
}
