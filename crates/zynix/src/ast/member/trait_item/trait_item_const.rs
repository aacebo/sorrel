use super::TraitItem;
use crate::ast::{Attribute, Expr, Generics, Ident, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::Const;
use crate::token::punct::{Colon, Eq, Semi};
use crate::token::{LexError, ToTokens};
use crate::{Parse, Span, TokenStream};

#[doc = "A constant item inside a trait definition (`const NAME: Type;` or `const NAME: Type = expr;`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemConst {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
    pub default: Option<Expr>,
}

impl Parse for TraitItemConst {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();
        match TraitItem::parse(stream)? {
            TraitItem::Const(v) => Ok(v),
            _ => Err(LexError::new(at).message("expected trait const").into()),
        }
    }
}

impl ToTokens for TraitItemConst {
    fn to_tokens(&self, t: &mut TokenStream) {
        super::super::emit_attrs(&self.attrs, t);
        Const::default().to_tokens(t);
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        Colon::default().to_tokens(t);
        self.ty.to_tokens(t);
        if let Some(d) = &self.default {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }
        Semi::default().to_tokens(t);
    }
}
