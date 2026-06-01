use super::TraitItem;
use crate::ast::{Attribute, Signature, StmtBlock};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Semi;
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A method declaration or default implementation inside a trait definition."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemFn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub sig: Signature,
    pub default_body: Option<StmtBlock>,
}

impl Parse for TraitItemFn {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match TraitItem::parse(stream)? {
            TraitItem::Fn(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected trait fn").into()),
        }
    }
}

impl ToTokens for TraitItemFn {
    fn to_tokens(&self, t: &mut TokenStream) {
        super::super::emit_attrs(&self.attrs, t);
        self.sig.to_tokens(t);
        match &self.default_body {
            Some(b) => b.to_tokens(t),
            None => Semi::default().to_tokens(t),
        }
    }
}
