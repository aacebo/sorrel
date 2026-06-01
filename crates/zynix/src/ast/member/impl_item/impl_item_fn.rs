use super::ImplItem;
use crate::ast::{Attribute, Defaultness, Signature, StmtBlock, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A method or associated function inside an `impl` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: StmtBlock,
}

impl Parse for ImplItemFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match ImplItem::parse(stream)? {
            ImplItem::Fn(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected impl fn").into()),
        }
    }
}

impl ToTokens for ImplItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        super::super::emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.sig.to_tokens(t);
        self.body.to_tokens(t);
    }
}
