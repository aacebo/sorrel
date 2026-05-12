use super::*;
#[derive(Debug, Clone)]
pub struct Signature {
    pub span: crate::Span,
    pub constness: Constness,
    pub asyncness: Asyncness,
    pub unsafety: Unsafety,
    pub abi: Option<Abi>,
    pub ident: Ident,
    pub generics: Generics,
    pub inputs: crate::ast::Punctuated<FnParam, crate::token::Comma>,
    pub variadic: Option<Variadic>,
    pub output: ReturnType,
}
