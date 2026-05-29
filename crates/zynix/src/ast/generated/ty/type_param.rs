#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeParam {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub bounds: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
    pub default: Option<Type>,
}
