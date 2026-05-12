#[allow(unused)]
use super::*;
#[doc = "An identifier token (e.g. a variable name, type name, or keyword-like ident)."]
#[derive(Debug, Clone)]
pub struct Ident {
    pub span: crate::Span,
    pub text: String,
    pub raw: bool,
}
impl crate::ast::Visit for Ident {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_ident(self);
    }
}
impl crate::ast::Fold for Ident {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_ident(self)
    }
}
