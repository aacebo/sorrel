use super::*;
#[derive(Debug, Clone)]
pub struct Label {
    pub span: crate::Span,
    pub name: Lifetime,
}
