use super::*;
#[derive(Debug, Clone)]
pub struct FieldDef {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub mutability: Mutability,
    pub ident: Option<Ident>,
    pub ty: Type,
}
