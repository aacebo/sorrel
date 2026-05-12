use super::*;
#[derive(Debug, Clone)]
pub struct Abi {
    pub span: crate::Span,
    pub name: Option<String>,
}
