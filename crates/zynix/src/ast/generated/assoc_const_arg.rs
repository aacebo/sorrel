#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct AssocConstArg {
    pub span: crate::Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub expr: Expr,
}
