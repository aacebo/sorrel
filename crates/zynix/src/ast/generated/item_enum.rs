use super::*;
#[derive(Debug, Clone)]
pub struct ItemEnum {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub variants: crate::ast::Punctuated<Variant, crate::token::Comma>,
}
