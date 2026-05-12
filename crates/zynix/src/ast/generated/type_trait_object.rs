use super::*;
#[derive(Debug, Clone)]
pub struct TypeTraitObject {
    pub span: crate::Span,
    pub dyn_token: bool,
    pub bounds: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
}
