use super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::token::punct::Dot;
use crate::{Span, TokenStream};

#[doc = "A field access expression: `x.field`, `tuple.0`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprField {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<super::Expr>,
    pub member: Member,
}

impl ToTokens for ExprField {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.base.to_tokens(t);
        Dot::default().to_tokens(t);
        self.member.to_tokens(t);
    }
}
