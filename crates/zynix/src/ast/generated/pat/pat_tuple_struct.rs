#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatTupleStruct {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub elems: crate::ast::Punctuated<Pattern, crate::token::Comma>,
}
impl crate::ast::Visit for PatTupleStruct {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_pat_tuple_struct(self);
    }
}
impl crate::ast::Fold for PatTupleStruct {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_pat_tuple_struct(self)
    }
}
