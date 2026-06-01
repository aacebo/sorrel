use super::super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A binary operation expression: `a + b`, `x && y`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprBinary {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub left: Box<super::super::Expr>,
    pub op: BinOp,
    pub right: Box<super::super::Expr>,
}

impl ToTokens for ExprBinary {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.left.to_tokens(t);
        self.op.to_tokens(t);
        self.right.to_tokens(t);
    }
}
