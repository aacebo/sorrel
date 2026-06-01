use super::{emit_attrs, emit_group};
use crate::ast::*;
use crate::token::punct::Comma;
use crate::token::{Delim, ToTokens};
use crate::{Span, TokenStream};

#[doc = "A slice pattern, e.g. `[a, b, c]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatSlice {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub elems: Punctuated<Pattern, Comma>,
}

impl ToTokens for PatSlice {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        emit_group(Delim::Bracket, inner, t);
    }
}
