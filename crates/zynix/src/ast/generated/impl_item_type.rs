use super::*;
#[derive(Debug, Clone)]
pub struct ImplItemType {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
}
