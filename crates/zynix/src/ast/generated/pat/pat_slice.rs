#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatSlice {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub elems: crate::ast::Punctuated<Pattern, crate::token::Comma>,
}
impl crate::ast::Visit for PatSlice {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_pat_slice(self);
    }
}
impl crate::ast::Fold for PatSlice {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_pat_slice(self)
    }
}
