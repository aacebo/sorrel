use super::*;
#[derive(Debug, Clone)]
pub struct TypeGroup {
    pub span: crate::Span,
    pub elem: Box<Type>,
}
