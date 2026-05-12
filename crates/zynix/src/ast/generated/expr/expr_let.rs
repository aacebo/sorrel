#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprLet {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Box<Pattern>,
    pub expr: Box<Expr>,
}
impl crate::ast::Visit for ExprLet {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_let(self);
    }
}
impl crate::ast::Fold for ExprLet {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_let(self)
    }
}
