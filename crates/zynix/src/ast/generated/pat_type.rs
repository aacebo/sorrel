use super::*;
#[derive(Debug, Clone)]
pub struct PatType {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
    pub ty: Box<Type>,
}
