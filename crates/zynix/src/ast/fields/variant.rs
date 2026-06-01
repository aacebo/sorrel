use super::Fields;
use crate::ast::{Attribute, Expr, Ident};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::Eq;
use crate::{Parse, Span, TokenStream};

#[doc = "An enum variant (`Name`, `Name(T)`, `Name { x: T }`, `Name = 1`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Variant {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub fields: Fields,
    pub discriminant: Option<Expr>,
}

impl Parse for Variant {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let ident = stream.parse::<Ident>()?;
        let fields = stream.parse::<Fields>()?;
        let discriminant = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            Some(stream.parse::<Expr>()?)
        } else {
            None
        };
        Ok(Self {
            span: Span::default(),
            attrs,
            ident,
            fields,
            discriminant,
        })
    }
}

impl ToTokens for Variant {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.ident.to_tokens(t);
        self.fields.to_tokens(t);
        if let Some(d) = &self.discriminant {
            Eq::default().to_tokens(t);
            d.to_tokens(t);
        }
    }
}
