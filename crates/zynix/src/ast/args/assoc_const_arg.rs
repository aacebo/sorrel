use super::AngleArgs;
use crate::ast::{Expr, Ident};
use crate::token::ToTokens;
use crate::token::punct::Eq;
use crate::{Span, TokenStream};

#[doc = "An associated const binding (`N = 8`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocConstArg {
    pub span: Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub expr: Expr,
}

impl ToTokens for AssocConstArg {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }
        Eq::default().to_tokens(t);
        self.expr.to_tokens(t);
    }
}
