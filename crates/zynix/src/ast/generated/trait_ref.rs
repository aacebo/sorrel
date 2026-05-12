#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TraitRef {
    pub span: crate::Span,
    pub polarity: BoundPolarity,
    pub path: Path,
}
impl crate::ast::Visit for TraitRef {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_trait_ref(self);
    }
}
impl crate::ast::Fold for TraitRef {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_trait_ref(self)
    }
}
