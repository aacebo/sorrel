#[allow(unused)]
use super::*;
#[doc = "A named lifetime (e.g. `'a`, `'static`)."]
#[derive(Debug, Clone)]
pub struct Lifetime {
    pub span: crate::Span,
    pub apostrophe: crate::Span,
    pub ident: Ident,
}
