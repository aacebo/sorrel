use super::*;
#[derive(Debug, Clone)]
pub struct ExprTuple {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elems: crate::ast::Punctuated<Expr, crate::token::Comma>,
}
