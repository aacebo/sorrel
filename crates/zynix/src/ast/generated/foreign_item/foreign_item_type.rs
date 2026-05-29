#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ForeignItemType {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
}
