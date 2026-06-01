use super::super::emit_attrs;
use crate::ast::Attribute;
use crate::token::{Delim, Group, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[doc = "An index expression: `a[0]`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprIndex {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub base: Box<super::super::Expr>,
    pub index: Box<super::super::Expr>,
}

impl ToTokens for ExprIndex {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.base.to_tokens(t);
        let mut inner = TokenStream::new();
        self.index.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Bracket, inner)));
    }
}
