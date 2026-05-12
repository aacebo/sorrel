#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum TypeBound {
    Trait { value: TraitBound },
    Lifetime { value: Lifetime },
    Use { value: UseBound },
}
impl crate::ast::Visit for TypeBound {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            TypeBound::Trait { value } => {
                let _ = &value;
            }
            TypeBound::Lifetime { value } => {
                let _ = &value;
            }
            TypeBound::Use { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for TypeBound {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            TypeBound::Trait { value } => TypeBound::Trait { value },
            TypeBound::Lifetime { value } => TypeBound::Lifetime { value },
            TypeBound::Use { value } => TypeBound::Use { value },
        }
    }
}
