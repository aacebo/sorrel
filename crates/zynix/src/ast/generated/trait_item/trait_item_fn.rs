#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct TraitItemFn {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub sig: Signature,
    pub default_body: Option<Block>,
}
