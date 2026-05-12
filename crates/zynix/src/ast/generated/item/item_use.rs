#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemUse {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub tree: UseTree,
}
