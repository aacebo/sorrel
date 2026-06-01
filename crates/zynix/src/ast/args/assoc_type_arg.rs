use super::AngleArgs;
use crate::ast::{Ident, Type};
use crate::token::ToTokens;
use crate::token::punct::Eq;
use crate::{Span, TokenStream};

#[doc = "An associated type binding (`Item = T`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocTypeArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub ty: Type,
}

impl ToTokens for AssocTypeArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }
        Eq::default().to_tokens(t);
        self.ty.to_tokens(t);
    }
}
