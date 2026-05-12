#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub struct StmtMacro {
    pub span: crate::Span,
    pub attrs: Vec<Attribute>,
    pub mac: MacroCall,
    pub semi: bool,
}
