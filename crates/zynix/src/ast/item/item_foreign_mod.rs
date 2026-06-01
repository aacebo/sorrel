use super::{emit_attrs, emit_brace_items};
use crate::ast::{Abi, Attribute, ForeignItem, Unsafety};
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "An `extern` block (`extern \"C\" { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemForeignMod {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub unsafety: Unsafety,
    pub abi: Abi,
    pub items: Vec<ForeignItem>,
}

impl ToTokens for ItemForeignMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.unsafety.to_tokens(t);
        self.abi.to_tokens(t);
        emit_brace_items(&self.items, t);
    }
}
