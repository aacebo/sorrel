#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemForeignMod {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub unsafety: Unsafety,
    pub abi: Abi,
    pub items: Vec<ForeignItem>,
}
impl crate::ast::Visit for ItemForeignMod {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_item_foreign_mod(self);
    }
}
impl crate::ast::Fold for ItemForeignMod {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_item_foreign_mod(self)
    }
}
