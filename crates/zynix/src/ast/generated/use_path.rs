use super::*;
#[derive(Debug, Clone)]
pub struct UsePath {
    pub span: crate::Span,
    pub ident: Ident,
    pub tree: Box<UseTree>,
}
