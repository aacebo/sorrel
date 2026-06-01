use super::ImplItem;
use crate::ast::{Attribute, MacroCall};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Semi;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A macro invocation inside an `impl` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ImplItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

impl Parse for ImplItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match ImplItem::parse(stream)? {
            ImplItem::Macro(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected impl macro").into()),
        }
    }
}

impl ToTokens for ImplItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        super::super::emit_attrs(&self.attrs, t);
        self.mac.to_tokens(t);
        if self.semi {
            Semi::default().to_tokens(t);
        }
    }
}
