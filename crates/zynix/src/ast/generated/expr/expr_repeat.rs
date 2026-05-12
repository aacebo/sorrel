#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprRepeat {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elem: Box<Expr>,
    pub len: Box<Expr>,
}
impl crate::ast::Visit for ExprRepeat {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_repeat(self);
    }
}
impl crate::ast::Fold for ExprRepeat {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_repeat(self)
    }
}
