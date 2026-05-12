#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatTuple {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elems: crate::ast::Punctuated<Pattern, crate::token::Comma>,
}
impl crate::ast::Visit for PatTuple {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_pat_tuple(self);
    }
}
impl crate::ast::Fold for PatTuple {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_pat_tuple(self)
    }
}
