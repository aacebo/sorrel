use super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::{For, In};
use crate::{Span, TokenStream};

#[doc = "A for loop expression: `for pat in expr { ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprForLoop {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub pat: Box<Pattern>,
    pub expr: Box<super::Expr>,
    pub body: StmtBlock,
}

impl ToTokens for ExprForLoop {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        if let Some(l) = &self.label {
            l.to_tokens(t);
        }
        For::default().to_tokens(t);
        self.pat.to_tokens(t);
        In::default().to_tokens(t);
        self.expr.to_tokens(t);
        self.body.to_tokens(t);
    }
}
