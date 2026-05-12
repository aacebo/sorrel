#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct Variant {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub fields: Fields,
    pub discriminant: Option<Expr>,
}
impl crate::ast::Visit for Variant {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_variant(self);
    }
}
impl crate::ast::Fold for Variant {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_variant(self)
    }
}
