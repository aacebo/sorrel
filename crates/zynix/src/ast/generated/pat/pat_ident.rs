#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatIdent {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub by_ref: bool,
    pub mutability: Mutability,
    pub ident: Ident,
    pub subpat: Option<Box<Pattern>>,
}
