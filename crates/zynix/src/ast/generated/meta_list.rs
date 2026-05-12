#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct MetaList {
    pub span: crate::Span,
    pub path: Path,
    pub delimiter: DelimiterKind,
    pub tokens: crate::TokenStream,
}
impl crate::ast::Visit for MetaList {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_meta_list(self);
    }
}
impl crate::ast::Fold for MetaList {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_meta_list(self)
    }
}
