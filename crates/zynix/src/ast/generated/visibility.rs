use super::*;
#[doc = "The visibility of an item — controls whether it is accessible outside its defining module."]
#[derive(Debug, Clone)]
pub enum Visibility {
    Inherited {},
    Public {},
    Crate {},
    SelfValue {},
    Super {},
    Restricted { in_token: bool, path: Path },
}
