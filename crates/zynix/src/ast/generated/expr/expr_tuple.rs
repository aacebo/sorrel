#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprTuple {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elems: crate::ast::Punctuated<Expr, crate::token::Comma>,
}
impl crate::ast::Visit for ExprTuple {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_tuple(self);
    }
}
impl crate::ast::Fold for ExprTuple {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_tuple(self)
    }
}
