#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemForeignMod {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub unsafety: Unsafety,
    pub abi: Abi,
    pub items: Vec<ForeignItem>,
}
