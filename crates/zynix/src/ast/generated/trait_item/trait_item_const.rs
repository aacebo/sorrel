#[allow(unused)]
use super::*;
#[doc = "An associated constant declaration inside a trait or impl block (`const NAME: Type;` or `const NAME: Type = value;`)."]
#[derive(Debug, Clone)]
pub struct TraitItemConst {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
    pub default: Option<Expr>,
}
