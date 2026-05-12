#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitCStr {
    pub span: crate::Span,
    pub value: Vec<u8>,
}
impl crate::ast::Visit for LitCStr {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_c_str(self);
    }
}
impl crate::ast::Fold for LitCStr {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_c_str(self)
    }
}
