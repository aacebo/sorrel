#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct StmtMacro {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}
impl crate::ast::Visit for StmtMacro {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_stmt_macro(self);
    }
}
impl crate::ast::Fold for StmtMacro {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_stmt_macro(self)
    }
}
