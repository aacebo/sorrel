#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct LitStr {
    pub span: crate::Span,
    pub value: String,
}
