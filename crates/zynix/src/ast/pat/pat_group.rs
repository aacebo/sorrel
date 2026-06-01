use super::{emit_attrs, emit_group};
use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A grouped pattern (used internally by the parser for grouping token trees)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatGroup {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
}

impl ToTokens for PatGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.pat.to_tokens(t);
    }
}
