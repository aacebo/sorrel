use super::emit_attrs;
use crate::ast::Attribute;
use crate::token::ToTokens;
use crate::token::keyword::Await as KwAwait;
use crate::token::punct::Dot;
use crate::{Span, TokenStream};

#[doc = "An await expression: `expr.await`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprAwait {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<super::Expr>,
}

impl ToTokens for ExprAwait {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.base.to_tokens(t);
        Dot::default().to_tokens(t);
        KwAwait::default().to_tokens(t);
    }
}
