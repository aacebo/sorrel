use super::*;
#[derive(Debug, Clone)]
pub struct Receiver {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub reference: bool,
    pub lifetime: Option<Lifetime>,
    pub mutability: Mutability,
}
