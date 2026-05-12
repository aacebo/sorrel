use super::*;
#[derive(Debug, Clone)]
pub enum TypeBound {
    Trait { value: TraitBound },
    Lifetime { value: Lifetime },
    Use { value: UseBound },
}
