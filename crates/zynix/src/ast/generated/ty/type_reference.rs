#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeReference {
    pub span: crate::Span,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
    pub elem: Box<Type>,
}
impl crate::ast::Visit for TypeReference {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_type_reference(self);
    }
}
impl crate::ast::Fold for TypeReference {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_type_reference(self)
    }
}
