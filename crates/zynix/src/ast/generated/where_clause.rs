#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub span: crate::Span,
    pub predicates: crate::ast::Punctuated<WherePredicate, crate::token::Comma>,
}
impl crate::ast::Visit for WhereClause {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_where_clause(self);
    }
}
impl crate::ast::Fold for WhereClause {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_where_clause(self)
    }
}
