#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ConstraintArg {
    pub span: crate::Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub bounds: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
}
