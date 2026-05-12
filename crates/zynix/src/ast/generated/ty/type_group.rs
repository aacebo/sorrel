#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeGroup {
    pub span: crate::Span,
    pub elem: Box<Type>,
}
impl crate::ast::Visit for TypeGroup {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_type_group(self);
    }
}
impl crate::ast::Fold for TypeGroup {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_type_group(self)
    }
}
