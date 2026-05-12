use super::*;
#[derive(Debug, Clone)]
pub struct ItemUnion {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: FieldsNamed,
}
