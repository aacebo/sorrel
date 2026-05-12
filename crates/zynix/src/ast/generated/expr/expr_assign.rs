#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprAssign {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
impl crate::ast::Visit for ExprAssign {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_assign(self);
    }
}
impl crate::ast::Fold for ExprAssign {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_assign(self)
    }
}
