#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum Fields {
    Named { value: FieldsNamed },
    Unnamed { value: FieldsUnnamed },
    Unit {},
}
impl crate::ast::Visit for Fields {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            Fields::Named { value } => {
                let _ = &value;
            }
            Fields::Unnamed { value } => {
                let _ = &value;
            }
            Fields::Unit {} => {}
        }
    }
}
impl crate::ast::Fold for Fields {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            Fields::Named { value } => Fields::Named { value },
            Fields::Unnamed { value } => Fields::Unnamed { value },
            Fields::Unit {} => Fields::Unit {},
        }
    }
}
