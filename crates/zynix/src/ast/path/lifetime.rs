use zynix_macros::{Parse, ToTokens};

use super::LifetimeName;
use crate::Span;
use crate::token::punct::Quote;

#[doc = "A named lifetime (e.g. `'a`, `'static`)."]
#[derive(Debug, Clone, Parse, ToTokens)]
pub struct Lifetime {
    #[parse(skip)]
    pub span: Span,
    #[parse(prefix = Quote)]
    pub ident: LifetimeName,
}
