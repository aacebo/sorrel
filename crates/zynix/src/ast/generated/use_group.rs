#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct UseGroup {
    pub span: crate::Span,
    pub items: crate::ast::Punctuated<UseTree, crate::token::Comma>,
}
impl crate::ast::Visit for UseGroup {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_use_group(self);
    }
}
impl crate::ast::Fold for UseGroup {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_use_group(self)
    }
}
