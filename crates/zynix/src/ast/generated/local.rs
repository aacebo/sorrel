use super::*;
#[derive(Debug, Clone)]
pub struct Local {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub ty: Option<Type>,
    pub init: Option<LocalInit>,
}
