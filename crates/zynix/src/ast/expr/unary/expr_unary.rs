use super::super::emit_attrs;
use crate::ast::*;
use crate::parse::ParseStream;
use crate::token::ToTokens;
use crate::token::punct::{Not, Star};
use crate::{Span, TokenStream};

#[doc = "A unary expression: `!x`, `-x`, `*x`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprUnary {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub op: UnOp,
    pub expr: Box<super::super::Expr>,
}

impl ExprUnary {
    /// Returns `true` if the stream starts with a prefix unary operator (`!`, `-`, `*`).
    pub(crate) fn is_prefix(stream: &mut ParseStream) -> bool {
        stream.peek::<Not>().is_some()
            || stream.peek::<crate::token::punct::Minus>().is_some()
            || stream.peek::<Star>().is_some()
    }
}

impl ToTokens for ExprUnary {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.op.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
