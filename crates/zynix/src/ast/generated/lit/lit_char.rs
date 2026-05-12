#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitChar {
    pub span: crate::Span,
    pub value: String,
}
impl crate::ast::Visit for LitChar {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_char(self);
    }
}
impl crate::ast::Fold for LitChar {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_char(self)
    }
}
