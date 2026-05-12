use super::*;
#[derive(Debug, Clone)]
pub struct ConstParam {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub ty: Type,
    pub default: Option<Expr>,
}
