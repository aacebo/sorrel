use super::*;
#[derive(Debug, Clone)]
pub struct Variant {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub fields: Fields,
    pub discriminant: Option<Expr>,
}
