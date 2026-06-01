use super::{emit_attrs, emit_group};
use crate::ast::*;
use crate::token::punct::Comma;
use crate::token::{Delim, ToTokens};
use crate::{Span, TokenStream};

#[doc = "A tuple-struct pattern, e.g. `Point(x, y)`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTupleStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub elems: Punctuated<Pattern, Comma>,
}

impl ToTokens for PatTupleStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.path.to_tokens(t);
        let mut inner = TokenStream::new();
        self.elems.to_tokens(&mut inner);
        emit_group(Delim::Paren, inner, t);
    }
}
