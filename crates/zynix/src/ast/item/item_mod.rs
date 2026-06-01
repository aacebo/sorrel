use super::{Item, emit_attrs, emit_brace_items};
use crate::ast::{Attribute, Ident, Unsafety, Visibility};
use crate::token::ToTokens;
use crate::token::keyword::Mod;
use crate::token::punct::Semi;
use crate::{Span, TokenStream};

#[doc = "A module item (`mod foo;` or `mod foo { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemMod {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub ident: Ident,
    pub content: Option<Vec<Item>>,
}

impl ToTokens for ItemMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        Mod::default().to_tokens(t);
        self.ident.to_tokens(t);
        match &self.content {
            Some(items) => emit_brace_items(items, t),
            None => Semi::default().to_tokens(t),
        }
    }
}
