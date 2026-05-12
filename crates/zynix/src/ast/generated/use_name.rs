#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct UseName {
    pub span: crate::Span,
    pub ident: Ident,
}
impl crate::ast::Visit for UseName {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_use_name(self);
    }
}
impl crate::ast::Fold for UseName {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_use_name(self)
    }
}
