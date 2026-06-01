use super::parse_bounds;
use crate::ast::{Lifetime, Punctuated};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::punct::Plus;
use crate::{Parse, Span, TokenStream};

#[doc = "A predicate in a `where` clause."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LifetimePredicate {
    pub span: Span,
    pub lifetime: Lifetime,
    pub bounds: Punctuated<Lifetime, Plus>,
}

impl Parse for LifetimePredicate {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let lifetime = stream.parse::<Lifetime>()?;
        let bounds = parse_bounds(stream)?;
        Ok(Self {
            span: Span::default(),
            lifetime,
            bounds,
        })
    }
}

impl ToTokens for LifetimePredicate {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.lifetime.to_tokens(t);
        super::emit_bounds(&self.bounds, t);
    }
}
