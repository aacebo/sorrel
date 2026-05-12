#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct Crate {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub items: Vec<Item>,
}
impl crate::ast::Visit for Crate {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_crate(self);
    }
}
impl crate::ast::Fold for Crate {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_crate(self)
    }
}
