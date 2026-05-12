#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprUnsafe {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub block: Block,
}
impl crate::ast::Visit for ExprUnsafe {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_unsafe(self);
    }
}
impl crate::ast::Fold for ExprUnsafe {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_unsafe(self)
    }
}
