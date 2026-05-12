#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct Local {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub ty: Option<Type>,
    pub init: Option<LocalInit>,
}
impl crate::ast::Visit for Local {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_local(self);
    }
}
impl crate::ast::Fold for Local {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_local(self)
    }
}
