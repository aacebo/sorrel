#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct MetaNameValue {
    pub span: crate::Span,
    pub path: Path,
    pub value: Expr,
}
impl crate::ast::Visit for MetaNameValue {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_meta_name_value(self);
    }
}
impl crate::ast::Fold for MetaNameValue {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_meta_name_value(self)
    }
}
