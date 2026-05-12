#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ExprAssignOp {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub left: Box<Expr>,
    pub op: AssignOp,
    pub right: Box<Expr>,
}
