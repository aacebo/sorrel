#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum UnOp {
    Deref,
    Not,
    Neg,
}
impl crate::ast::Visit for UnOp {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            UnOp::Deref => {}
            UnOp::Not => {}
            UnOp::Neg => {}
        }
    }
}
impl crate::ast::Fold for UnOp {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            UnOp::Deref => UnOp::Deref,
            UnOp::Not => UnOp::Not,
            UnOp::Neg => UnOp::Neg,
        }
    }
}
