#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemStatic {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Ident,
    pub ty: Type,
    pub expr: Expr,
}
impl crate::ast::Visit for ItemStatic {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_item_static(self);
    }
}
impl crate::ast::Fold for ItemStatic {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_item_static(self)
    }
}
