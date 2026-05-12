#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemMod {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Unsafety,
    pub ident: Ident,
    pub content: Option<Vec<Item>>,
}
impl crate::ast::Visit for ItemMod {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_item_mod(self);
    }
}
impl crate::ast::Fold for ItemMod {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_item_mod(self)
    }
}
