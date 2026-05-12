#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeParen {
    pub span: crate::Span,
    pub elem: Box<Type>,
}
impl crate::ast::Visit for TypeParen {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_type_paren(self);
    }
}
impl crate::ast::Fold for TypeParen {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_type_paren(self)
    }
}
