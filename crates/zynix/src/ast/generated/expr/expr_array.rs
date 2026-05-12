#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprArray {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elems: crate::ast::Punctuated<Expr, crate::token::Comma>,
}
