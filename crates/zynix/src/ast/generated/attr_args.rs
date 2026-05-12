#[allow(unused)]
use super::*;
#[derive(Debug, Clone)]
pub enum AttrArgs {
    Empty {},
    Delimited {
        delim: DelimiterKind,
        tokens: crate::TokenStream,
    },
    Meta {
        meta: Box<Meta>,
    },
}
