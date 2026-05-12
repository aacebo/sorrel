#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LifetimePredicate {
    pub span: crate::Span,
    pub lifetime: Lifetime,
    pub bounds: crate::ast::Punctuated<Lifetime, crate::token::Plus>,
}
impl crate::ast::Visit for LifetimePredicate {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lifetime_predicate(self);
    }
}
impl crate::ast::Fold for LifetimePredicate {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lifetime_predicate(self)
    }
}
