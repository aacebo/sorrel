#[allow(unused)]
use super::*;
#[doc = "A Rust attribute (`#[...]` or `#![...]`) applied to an item, expression, or statement."]
#[derive(Debug, Clone)]
pub struct Attribute {
    pub span: crate::Span,
    pub style: AttrStyle,
    pub path: Path,
    pub args: AttrArgs,
}
impl crate::ast::Visit for Attribute {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_attribute(self);
    }
}
impl crate::ast::Fold for Attribute {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_attribute(self)
    }
}
