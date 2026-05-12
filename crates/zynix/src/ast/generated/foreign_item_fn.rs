use super::*;
#[derive(Debug, Clone)]
pub struct ForeignItemFn {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
}
