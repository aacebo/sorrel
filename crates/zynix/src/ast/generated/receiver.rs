#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct Receiver {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub reference: bool,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
}
impl crate::ast::Visit for Receiver {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_receiver(self);
    }
}
impl crate::ast::Fold for Receiver {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_receiver(self)
    }
}
