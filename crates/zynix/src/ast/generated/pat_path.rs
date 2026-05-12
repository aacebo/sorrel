use super::*;
#[derive(Debug, Clone)]
pub struct PatPath {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
}
