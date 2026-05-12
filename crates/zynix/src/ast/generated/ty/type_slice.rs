#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeSlice {
    pub span: crate::Span,
    pub elem: Box<Type>,
}
