#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprMethodCall {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub receiver: Box<Expr>,
    pub method: Ident,
    pub turbofish: Option<AngleArgs>,
    pub args: crate::ast::Punctuated<Expr, crate::token::Comma>,
}
impl crate::ast::Visit for ExprMethodCall {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_expr_method_call(self);
    }
}
impl crate::ast::Fold for ExprMethodCall {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_expr_method_call(self)
    }
}
