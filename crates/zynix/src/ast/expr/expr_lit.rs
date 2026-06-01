use super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A literal expression: `1`, `\"hello\"`, `true`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprLit {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub lit: Lit,
}

impl ToTokens for ExprLit {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.lit.to_tokens(t);
    }
}
