use super::emit_attrs;
use crate::ast::*;
use crate::token::punct::Comma;
use crate::token::{Delim, Group, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[doc = "A function call expression: `f(a, b)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprCall {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub func: Box<super::Expr>,
    pub args: Punctuated<super::Expr, Comma>,
}

impl ToTokens for ExprCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.func.to_tokens(t);
        let mut inner = TokenStream::new();
        self.args.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
    }
}
