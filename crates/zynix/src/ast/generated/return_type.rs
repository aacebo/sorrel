#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum ReturnType {
    Default {},
    Type { value: Box<Type> },
}
impl crate::ast::Visit for ReturnType {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            ReturnType::Default {} => {}
            ReturnType::Type { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for ReturnType {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            ReturnType::Default {} => ReturnType::Default {},
            ReturnType::Type { value } => ReturnType::Type { value },
        }
    }
}
