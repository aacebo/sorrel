#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct AngleArgs {
    pub span: crate::Span,
    pub args: crate::ast::Punctuated<GenericArgument, crate::token::Comma>,
}
impl crate::ast::Visit for AngleArgs {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_angle_args(self);
    }
}
impl crate::ast::Fold for AngleArgs {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_angle_args(self)
    }
}
