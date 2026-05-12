#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LifetimeParam {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub lifetime: Lifetime,
    pub bounds: crate::ast::Punctuated<Lifetime, crate::token::Plus>,
}
impl crate::ast::Visit for LifetimeParam {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_lifetime_param(self);
    }
}
impl crate::ast::Fold for LifetimeParam {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_lifetime_param(self)
    }
}
