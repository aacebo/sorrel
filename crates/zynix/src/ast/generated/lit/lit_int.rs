#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitInt {
    pub span: crate::Span,
    pub digits: String,
    pub suffix: Option<Ident>,
}
impl crate::ast::Visit for LitInt {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_int(self);
    }
}
impl crate::ast::Fold for LitInt {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_int(self)
    }
}
