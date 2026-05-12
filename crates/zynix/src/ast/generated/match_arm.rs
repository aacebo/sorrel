#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Expr,
}
impl crate::ast::Visit for MatchArm {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_match_arm(self);
    }
}
impl crate::ast::Fold for MatchArm {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_match_arm(self)
    }
}
