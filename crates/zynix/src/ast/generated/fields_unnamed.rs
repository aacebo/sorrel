#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct FieldsUnnamed {
    pub span: crate::Span,
    pub fields: crate::ast::Punctuated<FieldDef, crate::token::Comma>,
}
