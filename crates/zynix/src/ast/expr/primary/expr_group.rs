use super::super::{emit_attrs, emit_group};
use crate::ast::Attribute;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A group expression (invisible delimiter wrapper used during macro expansion)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprGroup {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<super::super::Expr>,
}

impl ToTokens for ExprGroup {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        emit_group(&self.expr, t);
    }
}
