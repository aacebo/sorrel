use super::*;
#[derive(Debug, Clone)]
pub struct Variadic {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
}
