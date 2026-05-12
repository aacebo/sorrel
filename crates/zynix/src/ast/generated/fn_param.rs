#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum FnParam {
    Receiver { value: Box<Receiver> },
    Typed { value: Box<TypedParam> },
}
impl crate::ast::Visit for FnParam {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            FnParam::Receiver { value } => {
                let _ = &value;
            }
            FnParam::Typed { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for FnParam {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            FnParam::Receiver { value } => FnParam::Receiver { value },
            FnParam::Typed { value } => FnParam::Typed { value },
        }
    }
}
