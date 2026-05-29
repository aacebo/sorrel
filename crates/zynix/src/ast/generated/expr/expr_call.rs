#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprCall {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub func: Box<Expr>,
    pub args: crate::ast::Punctuated<Expr, crate::token::Comma>,
}
