use super::AngleArgs;
use crate::ast::{Ident, Punctuated, TypeBound};
use crate::token::ToTokens;
use crate::token::punct::{Colon, Plus};
use crate::{Span, TokenStream};

#[doc = "An associated type bound constraint (`Item: Bound`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstraintArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub bounds: Punctuated<TypeBound, Plus>,
}

impl ToTokens for ConstraintArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }
        Colon::default().to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
