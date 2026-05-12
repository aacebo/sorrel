#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ForeignItemMacro {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}
impl crate::ast::Visit for ForeignItemMacro {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_foreign_item_macro(self);
    }
}
impl crate::ast::Fold for ForeignItemMacro {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_foreign_item_macro(self)
    }
}
