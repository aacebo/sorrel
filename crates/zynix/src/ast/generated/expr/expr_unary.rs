#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprUnary {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub op: UnOp,
    pub expr: Box<Expr>,
}
impl crate::ast::Visit for ExprUnary {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_unary(self);
    }
}
impl crate::ast::Fold for ExprUnary {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_unary(self)
    }
}
