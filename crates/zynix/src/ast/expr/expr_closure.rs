use super::emit_attrs;
use crate::ast::*;
use crate::token::ToTokens;
use crate::token::keyword::Move;
use crate::token::punct::{Comma, Or};
use crate::{Span, TokenStream};

#[doc = "A closure expression: `|x| x`, `move || 1`, `async |x: u32| -> u32 { x }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprClosure {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub lifetimes: Option<BoundLifetimes>,
    pub constness: Constness,
    pub movability: Movability,
    pub asyncness: Asyncness,
    pub capture: bool,
    pub inputs: Punctuated<ClosureParam, Comma>,
    pub output: ReturnType,
    pub body: Box<super::Expr>,
}

impl ToTokens for ExprClosure {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.constness.to_tokens(t);
        self.movability.to_tokens(t);
        self.asyncness.to_tokens(t);
        if self.capture {
            Move::default().to_tokens(t);
        }
        Or::default().to_tokens(t);
        self.inputs.to_tokens(t);
        Or::default().to_tokens(t);
        self.output.to_tokens(t);
        self.body.to_tokens(t);
    }
}
