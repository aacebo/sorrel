use super::*;
#[derive(Debug, Clone)]
pub enum FnParam {
    Receiver { value: Receiver },
    Typed { value: TypedParam },
}
