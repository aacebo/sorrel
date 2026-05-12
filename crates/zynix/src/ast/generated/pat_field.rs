use super::*;
#[derive(Debug, Clone)]
pub struct PatField {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub member: Member,
    pub pat: Pattern,
    pub shorthand: bool,
}
