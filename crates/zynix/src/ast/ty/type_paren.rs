use zynix_macros::{Parse, ToTokens};

use super::Type;
use crate::Span;

#[doc = "A parenthesized type (e.g. `(T)`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
pub struct TypeParen {
    #[parse(skip)]
    pub span: Span,
    #[parse(paren)]
    pub elem: Box<Type>,
}
