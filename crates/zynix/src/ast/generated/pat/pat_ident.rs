#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatIdent {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub by_ref: bool,
    pub mutability: Mutability,
    pub ident: Ident,
    pub subpat: Option<Box<Pattern>>,
}
impl crate::ast::Visit for PatIdent {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_pat_ident(self);
    }
}
impl crate::ast::Fold for PatIdent {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_pat_ident(self)
    }
}
