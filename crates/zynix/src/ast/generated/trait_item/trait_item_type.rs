#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TraitItemType {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Ident,
    pub generics: Generics,
    pub bounds: crate::ast::Punctuated<TypeBound, crate::token::Plus>,
    pub default: Option<Type>,
}
impl crate::ast::Visit for TraitItemType {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_trait_item_type(self);
    }
}
impl crate::ast::Fold for TraitItemType {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_trait_item_type(self)
    }
}
