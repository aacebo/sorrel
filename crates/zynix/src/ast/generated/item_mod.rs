use super::*;
#[derive(Debug, Clone)]
pub struct ItemMod {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub ident: Ident,
    pub content: Option<Vec<Item>>,
}
