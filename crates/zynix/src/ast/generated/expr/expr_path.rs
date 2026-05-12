#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprPath {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
}
impl crate::ast::Visit for ExprPath {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_path(self);
    }
}
impl crate::ast::Fold for ExprPath {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_path(self)
    }
}
