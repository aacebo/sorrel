#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemTypeAlias {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub ty: Type,
}
impl crate::ast::Visit for ItemTypeAlias {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_item_type_alias(self);
    }
}
impl crate::ast::Fold for ItemTypeAlias {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_item_type_alias(self)
    }
}
