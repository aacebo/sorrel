#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct FieldsNamed {
    pub span: crate::Span,
    pub fields: crate::ast::Punctuated<FieldDef, crate::token::Comma>,
}
