use super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A block expression: `{ stmts }`, `'label: { stmts }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBlock {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub block: StmtBlock,
}

impl ToTokens for ExprBlock {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        if let Some(l) = &self.label {
            l.to_tokens(t);
        }
        self.block.to_tokens(t);
    }
}
