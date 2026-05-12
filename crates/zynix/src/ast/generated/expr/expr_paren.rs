#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprParen {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
}
impl crate::ast::Visit for ExprParen {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_paren(self);
    }
}
impl crate::ast::Fold for ExprParen {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_paren(self)
    }
}
