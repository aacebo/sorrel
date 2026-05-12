use super::*;
#[derive(Debug, Clone)]
pub struct PatSlice {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elems: crate::ast::Punctuated<Pattern, crate::token::Comma>,
}
