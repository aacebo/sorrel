use zynix_macros::{Parse, ToTokens};

use crate::Span;
use crate::ast::Lifetime;
use crate::token::punct::Colon;

#[doc = "A loop label (`'outer:`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Label {
    #[parse(skip)]
    pub span: Span,

    #[parse(suffix = Colon)]
    pub name: Lifetime,
}
