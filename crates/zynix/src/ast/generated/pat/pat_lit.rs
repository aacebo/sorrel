#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatLit {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub expr: Expr,
}
impl crate::ast::Visit for PatLit {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_pat_lit(self);
    }
}
impl crate::ast::Fold for PatLit {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_pat_lit(self)
    }
}
