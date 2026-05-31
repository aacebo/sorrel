use zynix_macros::{Parse, ToTokens};

use super::PathArguments;
use crate::Span;
use crate::ast::Ident;

#[derive(Debug, Clone, Parse, ToTokens)]
pub struct PathSegment {
    #[parse(skip)]
    pub span: Span,
    pub ident: Ident,
    pub args: PathArguments,
}
