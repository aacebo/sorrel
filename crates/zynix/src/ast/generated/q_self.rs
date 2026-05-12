#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct QSelf {
    pub span: crate::Span,
    pub ty: Box<Type>,
    pub position: usize,
}
