use super::*;
#[derive(Debug, Clone)]
pub struct ExprStruct {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub fields: crate::ast::Punctuated<FieldValue, crate::token::Comma>,
    pub rest: Option<Box<Expr>>,
}
