use super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A unary expression: `!x`, `-x`, `*x`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprUnary {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub op: UnOp,
    pub expr: Box<super::Expr>,
}

impl ToTokens for ExprUnary {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.op.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
