use super::*;
#[derive(Debug, Clone)]
pub struct ExprClosure {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub lifetimes: Option<BoundLifetimes>,
    pub constness: Constness,
    pub movability: Movability,
    pub asyncness: Asyncness,
    pub capture: bool,
    pub inputs: crate::ast::Punctuated<ClosureParam, crate::token::Comma>,
    pub output: ReturnType,
    pub body: Box<Expr>,
}
