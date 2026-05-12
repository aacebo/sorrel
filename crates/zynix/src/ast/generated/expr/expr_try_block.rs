#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprTryBlock {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub block: Block,
}
impl crate::ast::Visit for ExprTryBlock {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_try_block(self);
    }
}
impl crate::ast::Fold for ExprTryBlock {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_try_block(self)
    }
}
