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
