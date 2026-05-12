use super::*;
#[derive(Debug, Clone)]
pub struct BareFnArg {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
    pub ty: Type,
}
