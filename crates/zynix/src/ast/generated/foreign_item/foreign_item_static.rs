#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ForeignItemStatic {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Ident,
    pub ty: Type,
}
