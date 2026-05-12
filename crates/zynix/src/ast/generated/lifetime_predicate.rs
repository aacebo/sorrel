use super::*;
#[derive(Debug, Clone)]
pub struct LifetimePredicate {
    pub span: crate::Span,
    pub lifetime: Lifetime,
    pub bounds: crate::ast::Punctuated<Lifetime, crate::token::Plus>,
}
