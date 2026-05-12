#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitByte {
    pub span: crate::Span,
    pub value: u8,
}
impl crate::ast::Visit for LitByte {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lit_byte(self);
    }
}
impl crate::ast::Fold for LitByte {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lit_byte(self)
    }
}
