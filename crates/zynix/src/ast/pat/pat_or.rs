use super::{emit_attrs, emit_group};
use crate::ast::*;
use crate::token::ToTokens;
use crate::token::punct::Or as OrPunct;
use crate::{Span, TokenStream};

#[doc = "An or-pattern, e.g. `A | B | C`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatOr {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub cases: Punctuated<Pattern, OrPunct>,
}

impl ToTokens for PatOr {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.cases.to_tokens(t);
    }
}
