#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct UseBound {
    pub span: crate::Span,
    pub lifetimes: crate::ast::Punctuated<Lifetime, crate::token::Comma>,
}
impl crate::ast::Visit for UseBound {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_use_bound(self);
    }
}
impl crate::ast::Fold for UseBound {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_use_bound(self)
    }
}
