use zynix_macros::{Parse, ToTokens};

use super::Type;
use crate::Span;

#[doc = "A slice type (e.g. `[T]`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
pub struct TypeSlice {
    #[parse(skip)]
    pub span: Span,
    #[parse(bracket)]
    pub elem: Box<Type>,
}
