#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitBool {
    pub span: crate::Span,
    pub value: bool,
}
impl crate::ast::Visit for LitBool {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_bool(self);
    }
}
impl crate::ast::Fold for LitBool {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_bool(self)
    }
}
