#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitStr {
    pub span: crate::Span,
    pub value: String,
}
impl crate::ast::Visit for LitStr {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_str(self);
    }
}
impl crate::ast::Fold for LitStr {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_str(self)
    }
}
