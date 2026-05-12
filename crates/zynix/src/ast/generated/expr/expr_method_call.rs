#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprMethodCall {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub receiver: Box<Expr>,
    pub method: Ident,
    pub turbofish: Option<AngleArgs>,
    pub args: crate::ast::Punctuated<Expr, crate::token::Comma>,
}
