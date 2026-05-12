#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum WherePredicate {
    Lifetime { value: LifetimePredicate },
    Type { value: Box<TypePredicate> },
}
impl crate::ast::Visit for WherePredicate {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            WherePredicate::Lifetime { value } => {
                let _ = &value;
            }
            WherePredicate::Type { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for WherePredicate {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            WherePredicate::Lifetime { value } => WherePredicate::Lifetime { value },
            WherePredicate::Type { value } => WherePredicate::Type { value },
        }
    }
}
