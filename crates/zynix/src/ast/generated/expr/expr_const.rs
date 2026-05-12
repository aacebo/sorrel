#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprConst {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub block: Block,
}
impl crate::ast::Visit for ExprConst {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_const(self);
    }
}
impl crate::ast::Fold for ExprConst {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_const(self)
    }
}
