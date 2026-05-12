#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PathSegment {
    pub span: crate::Span,
    pub ident: Ident,
    pub args: PathArguments,
}
