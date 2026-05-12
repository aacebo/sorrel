#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum FnParam {
    Receiver { value: Box<Receiver> },
    Typed { value: Box<TypedParam> },
}
