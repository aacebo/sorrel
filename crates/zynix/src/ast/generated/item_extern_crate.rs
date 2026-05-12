use super::*;
#[derive(Debug, Clone)]
pub struct ItemExternCrate {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub rename: Option<Ident>,
}
