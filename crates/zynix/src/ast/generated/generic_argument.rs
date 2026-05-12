#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum GenericArgument {
    Lifetime { value: Lifetime },
    Type { value: Type },
    Const { value: Expr },
    AssocType { value: AssocTypeArg },
    AssocConst { value: AssocConstArg },
    Constraint { value: ConstraintArg },
}
impl crate::ast::Visit for GenericArgument {
    fn visit(&self, visitor: &mut impl crate::ast::Visitor) {
        match self {
            GenericArgument::Lifetime { value } => {
                let _ = &value;
            }
            GenericArgument::Type { value } => {
                let _ = &value;
            }
            GenericArgument::Const { value } => {
                let _ = &value;
            }
            GenericArgument::AssocType { value } => {
                let _ = &value;
            }
            GenericArgument::AssocConst { value } => {
                let _ = &value;
            }
            GenericArgument::Constraint { value } => {
                let _ = &value;
            }
        }
    }
}
impl crate::ast::Fold for GenericArgument {
    fn fold(self, folder: &mut impl crate::ast::Folder) -> Self {
        match self {
            GenericArgument::Lifetime { value } => GenericArgument::Lifetime { value },
            GenericArgument::Type { value } => GenericArgument::Type { value },
            GenericArgument::Const { value } => GenericArgument::Const { value },
            GenericArgument::AssocType { value } => GenericArgument::AssocType { value },
            GenericArgument::AssocConst { value } => GenericArgument::AssocConst { value },
            GenericArgument::Constraint { value } => GenericArgument::Constraint { value },
        }
    }
}
