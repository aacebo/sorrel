use super::*;
#[derive(Debug, Clone)]
pub struct TypeTuple {
    pub span: crate::Span,
    pub elems: crate::ast::Punctuated<Type, crate::token::Comma>,
}
