#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct BoundLifetimes {
    pub span: crate::Span,
    pub params: crate::ast::Punctuated<LifetimeParam, crate::token::Comma>,
}
