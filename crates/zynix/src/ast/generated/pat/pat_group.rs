#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatGroup {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
}
