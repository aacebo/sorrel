#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LocalInit {
    pub span: crate::Span,
    pub expr: Expr,
    pub diverge: Option<Box<Expr>>,
}
impl crate::ast::Visit for LocalInit {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_local_init(self);
    }
}
impl crate::ast::Fold for LocalInit {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_local_init(self)
    }
}
