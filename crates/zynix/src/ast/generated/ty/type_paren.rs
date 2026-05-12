#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeParen {
    pub span: crate::Span,
    pub elem: Box<Type>,
}
