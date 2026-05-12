use super::*;
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub span: crate::Span,
    pub predicates: crate::ast::Punctuated<WherePredicate, crate::token::Comma>,
}
