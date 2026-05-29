#[allow(unused)]
use super::*;
#[doc = "A top-level constant item (`const NAME: Type = value;`)."]
#[derive(Debug, Clone)]
pub struct ItemConst {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
    pub expr: Expr,
}
