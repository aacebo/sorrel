#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ImplItemType {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
}
impl crate::ast::Visit for ImplItemType {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_impl_item_type(self);
    }
}
impl crate::ast::Fold for ImplItemType {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_impl_item_type(self)
    }
}
