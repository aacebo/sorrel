#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ImplItemFn {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: Block,
}
impl crate::ast::Visit for ImplItemFn {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_impl_item_fn(self);
    }
}
impl crate::ast::Fold for ImplItemFn {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_impl_item_fn(self)
    }
}
