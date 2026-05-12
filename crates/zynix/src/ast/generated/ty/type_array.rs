#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeArray {
    pub span: crate::Span,
    pub elem: Box<Type>,
    pub len: Expr,
}
impl crate::ast::Visit for TypeArray {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_type_array(self);
    }
}
impl crate::ast::Fold for TypeArray {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_type_array(self)
    }
}
