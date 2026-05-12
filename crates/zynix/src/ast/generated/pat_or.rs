use super::*;
#[derive(Debug, Clone)]
pub struct PatOr {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub cases: crate::ast::Punctuated<Pattern, crate::token::Or>,
}
