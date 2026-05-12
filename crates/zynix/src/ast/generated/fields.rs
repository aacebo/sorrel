#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum Fields {
    Named { value: FieldsNamed },
    Unnamed { value: FieldsUnnamed },
    Unit {},
}
