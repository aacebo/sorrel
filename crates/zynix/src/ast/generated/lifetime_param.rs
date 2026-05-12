use super::*;
#[derive(Debug, Clone)]
pub struct LifetimeParam {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub lifetime: Lifetime,
    pub bounds: crate::ast::Punctuated<Lifetime, crate::token::Plus>,
}
