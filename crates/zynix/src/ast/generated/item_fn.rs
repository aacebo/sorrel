use super::*;
#[derive(Debug, Clone)]
pub struct ItemFn {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub defaultness: Defaultness,
    pub sig: Signature,
    pub body: Block,
}
