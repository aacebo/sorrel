#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct Crate {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub items: Vec<Item>,
}
