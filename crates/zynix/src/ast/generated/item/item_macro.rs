#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct ItemMacro {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub ident: Option<Ident>,
    pub mac: MacroCall,
    pub semi: bool,
}
