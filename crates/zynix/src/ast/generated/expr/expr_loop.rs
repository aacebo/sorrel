#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprLoop {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub body: Block,
}
