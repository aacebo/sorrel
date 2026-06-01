use super::ForeignItem;
use crate::ast::{Attribute, Signature, Visibility};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Semi;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A foreign function declaration inside an `extern` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
}

impl Parse for ForeignItemFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match ForeignItem::parse(stream)? {
            ForeignItem::Fn(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected foreign fn").into()),
        }
    }
}

impl ToTokens for ForeignItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        super::super::emit_attrs(&self.attrs, t);
        self.vis.to_tokens(t);
        self.sig.to_tokens(t);
        Semi::default().to_tokens(t);
    }
}
