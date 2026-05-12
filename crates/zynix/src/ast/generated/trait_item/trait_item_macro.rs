#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TraitItemMacro {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}
impl crate::ast::Visit for TraitItemMacro {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_trait_item_macro(self);
    }
}
impl crate::ast::Fold for TraitItemMacro {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_trait_item_macro(self)
    }
}
