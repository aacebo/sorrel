#[allow(unused)]
use crate::ast::*;

#[derive(Debug, Clone)]
pub struct TypeImplTrait {
    pub span: crate::Span,
    pub bounds: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
}
