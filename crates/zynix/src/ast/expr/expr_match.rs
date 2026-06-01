use super::emit_attrs;
use crate::ast::*;
use crate::token::keyword::Match;
use crate::token::{Delim, Group, ToTokens};
use crate::{Span, TokenStream, TokenTree};

#[doc = "A match expression: `match x { pat => expr, ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMatch {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<super::Expr>,
    pub arms: Vec<MatchArm>,
}

impl ToTokens for ExprMatch {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        Match::default().to_tokens(t);
        self.expr.to_tokens(t);
        let mut inner = TokenStream::new();
        for arm in &self.arms {
            arm.to_tokens(&mut inner);
        }
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}
