#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum ClosureParam {
    Typed { pat: Box<Pattern>, ty: Box<Type> },
    Inferred { pat: Box<Pattern> },
}
