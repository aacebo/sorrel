#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprCast {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
    pub ty: Box<Type>,
}
impl crate::ast::Visit for ExprCast {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_cast(self);
    }
}
impl crate::ast::Fold for ExprCast {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_cast(self)
    }
}
