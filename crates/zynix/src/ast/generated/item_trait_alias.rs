use super::*;
#[derive(Debug, Clone)]
pub struct ItemTraitAlias {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
}
