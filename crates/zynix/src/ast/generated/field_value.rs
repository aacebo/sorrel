#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct FieldValue {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub member: Member,
    pub expr: Expr,
    pub shorthand: bool,
}
