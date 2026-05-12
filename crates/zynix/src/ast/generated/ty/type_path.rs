#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypePath {
    pub span: crate::Span,
    pub qself: Option<QSelf>,
    pub path: Path,
}
