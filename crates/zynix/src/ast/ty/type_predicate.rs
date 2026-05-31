#[allow(unused)]
use crate::ast::*;

#[derive(Debug, Clone)]
pub struct TypePredicate {
    pub span: crate::Span,
    pub lifetimes: Option<BoundLifetimes>,
    pub bounded_ty: Type,
    pub bounds: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
}
