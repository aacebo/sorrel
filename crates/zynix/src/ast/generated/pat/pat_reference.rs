#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatReference {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub mutability: Mutability,
    pub pat: Box<Pattern>,
}
impl crate::ast::Visit for PatReference {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_pat_reference(self);
    }
}
impl crate::ast::Fold for PatReference {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_pat_reference(self)
    }
}
