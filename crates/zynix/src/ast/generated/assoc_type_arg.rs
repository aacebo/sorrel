#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct AssocTypeArg {
    pub span: crate::Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub ty: Type,
}
impl crate::ast::Visit for AssocTypeArg {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_assoc_type_arg(self);
    }
}
impl crate::ast::Fold for AssocTypeArg {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_assoc_type_arg(self)
    }
}
