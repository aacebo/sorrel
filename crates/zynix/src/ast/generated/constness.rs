#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum Constness {
    NoConst,
    Const,
}
impl crate::ast::Visit for Constness {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Constness::NoConst => {}
            Constness::Const => {}
        }
    }
}
impl crate::ast::Fold for Constness {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Constness::NoConst => Constness::NoConst,
            Constness::Const => Constness::Const,
        }
    }
}
