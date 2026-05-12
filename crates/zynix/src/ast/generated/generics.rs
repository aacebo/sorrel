#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct Generics {
    pub span: crate::Span,
    pub params: crate::ast::Punctuated<GenericParam, crate::token::Comma>,
    pub where_clause: Option<WhereClause>,
}
