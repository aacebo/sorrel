#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct UseBound {
    pub span: crate::Span,
    pub lifetimes: crate::ast::Punctuated<Lifetime, crate::token::Comma>,
}
