#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemStruct {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub fields: Fields,
}
impl crate::ast::Visit for ItemStruct {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_item_struct(self);
    }
}
impl crate::ast::Fold for ItemStruct {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_item_struct(self)
    }
}
