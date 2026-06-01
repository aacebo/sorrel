use super::TraitItem;
use crate::ast::{Attribute, MacroCall};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Semi;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A macro invocation inside a trait definition."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemMacro {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}

impl Parse for TraitItemMacro {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match TraitItem::parse(stream)? {
            TraitItem::Macro(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected trait macro").into()),
        }
    }
}

impl ToTokens for TraitItemMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        super::super::emit_attrs(&self.attrs, t);
        self.mac.to_tokens(t);
        if self.semi {
            Semi::default().to_tokens(t);
        }
    }
}
