#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct UseRename {
    pub span: crate::Span,
    pub ident: Ident,
    pub rename: Ident,
}
impl crate::ast::Visit for UseRename {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_use_rename(self);
    }
}
impl crate::ast::Fold for UseRename {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_use_rename(self)
    }
}
