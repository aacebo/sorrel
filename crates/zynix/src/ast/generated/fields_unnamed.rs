#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct FieldsUnnamed {
    pub span: crate::Span,
    pub fields: crate::ast::Punctuated<FieldDef, crate::token::Comma>,
}
impl crate::ast::Visit for FieldsUnnamed {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_fields_unnamed(self);
    }
}
impl crate::ast::Fold for FieldsUnnamed {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_fields_unnamed(self)
    }
}
