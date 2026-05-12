#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct AngleArgs {
    pub span: crate::Span,
    pub args: crate::ast::Punctuated<GenericArgument, crate::token::Comma>,
}
