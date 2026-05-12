#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct PatTupleStruct {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub elems: crate::ast::Punctuated<Pattern, crate::token::Comma>,
}
