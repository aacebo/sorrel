use super::*;
#[derive(Debug, Clone)]
pub enum ReturnType {
    Default {},
    Type { value: Box<Type> },
}
