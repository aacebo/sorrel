use super::emit_attrs;
use crate::ast::*;
use crate::token::punct::{Comma, Dot};
use crate::token::{Delim, Group, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[doc = "A method call expression: `receiver.method(args)`, `x.collect::<Vec<_>>()`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMethodCall {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub receiver: Box<super::Expr>,
    pub method: Ident,
    pub turbofish: Option<AngleArgs>,
    pub args: Punctuated<super::Expr, Comma>,
}

impl ToTokens for ExprMethodCall {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.receiver.to_tokens(t);
        Dot::default().to_tokens(t);
        self.method.to_tokens(t);
        if let Some(tf) = &self.turbofish {
            tf.to_tokens(t);
        }
        let mut inner = TokenStream::new();
        self.args.to_tokens(&mut inner);
        t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
    }
}
