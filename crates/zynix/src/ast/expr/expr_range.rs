use super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A range expression: `0..10`, `a..=b`, `..`, `a..`, `..b`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprRange {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub start: Option<Box<super::Expr>>,
    pub limits: RangeLimits,
    pub end: Option<Box<super::Expr>>,
}

impl ToTokens for ExprRange {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        if let Some(s) = &self.start {
            s.to_tokens(t);
        }
        self.limits.to_tokens(t);
        if let Some(e) = &self.end {
            e.to_tokens(t);
        }
    }
}
