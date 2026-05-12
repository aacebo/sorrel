#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum Movability {
    Movable,
    Static,
}
impl crate::ast::Visit for Movability {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Movability::Movable => {}
            Movability::Static => {}
        }
    }
}
impl crate::ast::Fold for Movability {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Movability::Movable => Movability::Movable,
            Movability::Static => Movability::Static,
        }
    }
}
