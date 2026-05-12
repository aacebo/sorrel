#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemMacro {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Option<Ident>,
    pub mac: MacroCall,
    pub semi: bool,
}
impl crate::ast::Visit for ItemMacro {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_item_macro(self);
    }
}
impl crate::ast::Fold for ItemMacro {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_item_macro(self)
    }
}
