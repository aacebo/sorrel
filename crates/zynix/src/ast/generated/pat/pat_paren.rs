#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatParen {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
}
