use super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::{Async, Move};
use crate::{Span, TokenStream};

#[doc = "An async block expression: `async { ... }`, `async move { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAsync {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub capture: bool,
    pub block: StmtBlock,
}

impl ToTokens for ExprAsync {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Async::default().to_tokens(t);
        if self.capture {
            Move::default().to_tokens(t);
        }
        self.block.to_tokens(t);
    }
}
