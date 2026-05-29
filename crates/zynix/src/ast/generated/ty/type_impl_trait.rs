#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeImplTrait {
    pub span: crate::Span,
    pub bounds: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
}
