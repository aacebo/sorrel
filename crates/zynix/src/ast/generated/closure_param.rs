use super::*;
#[derive(Debug, Clone)]
pub enum ClosureParam {
    Typed { pat: Pattern, ty: Type },
    Inferred { pat: Pattern },
}
