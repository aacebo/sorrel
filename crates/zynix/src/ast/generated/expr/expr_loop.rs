#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprLoop {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub body: Block,
}
impl crate::ast::Visit for ExprLoop {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_loop(self);
    }
}
impl crate::ast::Fold for ExprLoop {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_loop(self)
    }
}
