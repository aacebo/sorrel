use super::emit_attrs;
use crate::ast::Attribute;
use crate::token::ToTokens;
use crate::token::keyword::Return;
use crate::{Span, TokenStream};

#[doc = "A return expression: `return`, `return expr`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprReturn {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Option<Box<super::Expr>>,
}

impl ToTokens for ExprReturn {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Return::default().to_tokens(t);
        if let Some(e) = &self.expr {
            e.to_tokens(t);
        }
    }
}
