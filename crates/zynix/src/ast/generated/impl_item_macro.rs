use super::*;
#[derive(Debug, Clone)]
pub struct ImplItemMacro {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}
