use super::super::emit_attrs;
use crate::ast::*;
use crate::parse::ParseStream;
use crate::token::{Delim, TokenTree};
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A block expression: `{ stmts }`, `'label: { stmts }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBrace {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub block: StmtBlock,
}

impl ExprBrace {
    /// Returns `true` when the token at position 1 (peek-ahead) is a brace group.
    pub(crate) fn is_next(stream: &ParseStream) -> bool {
        matches!(stream.nth(1), Some(TokenTree::Group(g)) if g.delim() == Delim::Brace)
    }
}

impl ToTokens for ExprBrace {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        if let Some(l) = &self.label {
            l.to_tokens(t);
        }
        self.block.to_tokens(t);
    }
}
