#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum GenericParam {
    Lifetime { value: LifetimeParam },
    Type { value: TypeParam },
    Const { value: ConstParam },
}
