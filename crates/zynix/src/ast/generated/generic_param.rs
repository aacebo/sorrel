#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum GenericParam {
    Lifetime { value: LifetimeParam },
    Type { value: TypeParam },
    Const { value: ConstParam },
}
impl crate::ast::Visit for GenericParam {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            GenericParam::Lifetime { value } => {
                let _ = &value;
            }
            GenericParam::Type { value } => {
                let _ = &value;
            }
            GenericParam::Const { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for GenericParam {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            GenericParam::Lifetime { value } => GenericParam::Lifetime { value },
            GenericParam::Type { value } => GenericParam::Type { value },
            GenericParam::Const { value } => GenericParam::Const { value },
        }
    }
}
