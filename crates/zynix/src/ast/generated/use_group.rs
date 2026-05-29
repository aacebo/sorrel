#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct UseGroup {
    pub span: crate::Span,
    pub items: crate::ast::Punctuated<UseTree, crate::token::Comma>,
}
