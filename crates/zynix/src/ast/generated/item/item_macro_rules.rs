#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemMacroRules {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub rules: crate::TokenStream,
}
