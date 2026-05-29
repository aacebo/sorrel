#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct AssocTypeArg {
    pub span: crate::Span,
    pub ident: Ident,
    pub generics: Option<AngleArgs>,
    pub ty: Type,
}
