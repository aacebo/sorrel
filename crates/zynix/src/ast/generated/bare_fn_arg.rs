#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct BareFnArg {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub name: Option<Ident>,
    pub ty: Type,
}
impl crate::ast::Visit for BareFnArg {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_bare_fn_arg(self);
    }
}
impl crate::ast::Fold for BareFnArg {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_bare_fn_arg(self)
    }
}
