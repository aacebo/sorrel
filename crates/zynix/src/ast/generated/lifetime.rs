#[allow(unused)]
use super::*;
#[doc = "A named lifetime (e.g. `'a`, `'static`)."]
#[derive(Debug, Clone)]
pub struct Lifetime {
    pub span: crate::Span,
    pub apostrophe: crate::Span,
    pub ident: Ident,
}
impl crate::ast::Visit for Lifetime {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lifetime(self);
    }
}
impl crate::ast::Fold for Lifetime {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lifetime(self)
    }
}
