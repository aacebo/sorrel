use super::{emit_attrs, emit_group};
use crate::ast::*;
use crate::token::{Delim, ToTokens};
use crate::{Span, TokenStream};

#[doc = "A parenthesized pattern, e.g. `(A | B)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatParen {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
}

impl ToTokens for PatParen {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.pat.to_tokens(&mut inner);
        emit_group(Delim::Paren, inner, t);
    }
}
