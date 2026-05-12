#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct FieldsNamed {
    pub span: crate::Span,
    pub fields: crate::ast::Punctuated<FieldDef, crate::token::Comma>,
}
impl crate::ast::Visit for FieldsNamed {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_fields_named(self);
    }
}
impl crate::ast::Fold for FieldsNamed {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_fields_named(self)
    }
}
