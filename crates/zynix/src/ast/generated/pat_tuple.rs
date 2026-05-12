use super::*;
#[derive(Debug, Clone)]
pub struct PatTuple {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elems: crate::ast::Punctuated<Pattern, crate::token::Comma>,
}
