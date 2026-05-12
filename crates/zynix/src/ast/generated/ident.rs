use super::*;
#[doc = "An identifier token (e.g. a variable name, type name, or keyword-like ident)."]
#[derive(Debug, Clone)]
pub struct Ident {
    pub span: crate::Span,
    pub text: String,
    pub raw: bool,
}
