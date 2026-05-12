#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TypeBareFn {
    pub span: crate::Span,
    pub lifetimes: Option<BoundLifetimes>,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub inputs: crate::ast::Punctuated<BareFnArg, crate::token::Comma>,
    pub variadic: Option<Variadic>,
    pub output: ReturnType,
}
impl crate::ast::Visit for TypeBareFn {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        visitor.visit_type_bare_fn(self);
    }
}
impl crate::ast::Fold for TypeBareFn {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        folder.fold_type_bare_fn(self)
    }
}
