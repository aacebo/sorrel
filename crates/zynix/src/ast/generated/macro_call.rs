#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct MacroCall {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub path: Path,
    pub delimiter: DelimiterKind,
    pub tokens: crate::TokenStream,
}
impl crate::ast::Visit for MacroCall {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_macro_call(self);
    }
}
impl crate::ast::Fold for MacroCall {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_macro_call(self)
    }
}
