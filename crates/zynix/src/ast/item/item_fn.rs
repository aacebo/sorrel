use super::emit_attrs;
use crate::ast::{Attribute, Defaultness, Signature, StmtBlock, Visibility};
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A free function item (`fn name(...) -> T { ... }`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: StmtBlock,
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
    }
}
