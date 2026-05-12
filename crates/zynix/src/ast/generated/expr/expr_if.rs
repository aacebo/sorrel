#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprIf {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub cond: Box<Expr>,
    pub then_branch: Block,
    pub else_branch: Option<Box<Expr>>,
}
impl crate::ast::Visit for ExprIf {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_if(self);
    }
}
impl crate::ast::Fold for ExprIf {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_if(self)
    }
}
