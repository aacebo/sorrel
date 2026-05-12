#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprForLoop {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub label: Option<Label>,
    pub pat: Box<Pattern>,
    pub expr: Box<Expr>,
    pub body: Block,
}
impl crate::ast::Visit for ExprForLoop {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_for_loop(self);
    }
}
impl crate::ast::Fold for ExprForLoop {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_for_loop(self)
    }
}
