use super::ForeignItem;
use crate::ast::{Attribute, MacroCall};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Semi;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A macro invocation inside an `extern` block."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ForeignItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

impl Parse for ForeignItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match ForeignItem::parse(stream)? {
            ForeignItem::Macro(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected foreign macro").into()),
        }
    }
}

impl ToTokens for ForeignItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        super::super::emit_attrs(&self.attrs, t);
        self.mac.to_tokens(t);
        if self.semi {
            Semi::default().to_tokens(t);
        }
    }
}
