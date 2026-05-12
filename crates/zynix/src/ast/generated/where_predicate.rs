use super::*;
#[derive(Debug, Clone)]
pub enum WherePredicate {
    Lifetime { value: LifetimePredicate },
    Type { value: TypePredicate },
}
