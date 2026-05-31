#[allow(unused)]
use crate::ast::*;

#[derive(Debug, Clone)]
pub struct TypeArray {
    pub span: crate::Span,
    pub elem: Box<Type>,
    pub len: Expr,
}
