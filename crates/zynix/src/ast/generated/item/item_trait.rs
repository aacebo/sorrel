#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemTrait {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub auto: bool,
    pub ident: Ident,
    pub generics: Generics,
    pub supertraits: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
    pub items: Vec<TraitItem>,
}
