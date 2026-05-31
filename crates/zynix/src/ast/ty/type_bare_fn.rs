#[allow(unused)]
use crate::ast::*;

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
