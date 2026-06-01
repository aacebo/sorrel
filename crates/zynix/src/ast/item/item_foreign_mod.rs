use super::{emit_attrs, emit_brace_items};
use crate::ast::{Abi, Attribute, ForeignItem, Unsafety};
use crate::parse::{ParseError, ParseStream};
use crate::token::{Delim, ToTokens};
use crate::{Parse, Span, TokenStream};

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

impl Parse for ItemForeignMod {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let unsafety = stream.parse::<Unsafety>()?;
        let abi = stream.parse::<Abi>()?;
        let group = stream.parse_group(Delim::Brace)?;
        let mut inner = group.parse();
        let items = inner.parse_vec::<ForeignItem>()?;
        Ok(ItemForeignMod {
            span: Span::default(),
            attrs,
            unsafety,
            abi,
            items,
        })
    }
}

impl ToTokens for ItemForeignMod {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.unsafety.to_tokens(t);
        self.abi.to_tokens(t);
        emit_brace_items(&self.items, t);
    }
}
