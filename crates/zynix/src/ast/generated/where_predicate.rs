#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum WherePredicate {
    Lifetime { value: LifetimePredicate },
    Type { value: Box<TypePredicate> },
}
