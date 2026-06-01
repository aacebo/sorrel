use super::{emit_attrs, emit_group};
use crate::ast::pat::PatField;
use crate::ast::*;
use crate::token::punct::{Comma, DotDot};
use crate::token::{Delim, ToTokens};
use crate::{Span, TokenStream};

#[doc = "A struct pattern, e.g. `Point { x, y }` or `Point { x, .. }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatStruct {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub fields: Punctuated<PatField, Comma>,
    pub rest: bool,
}

impl ToTokens for PatStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        emit_attrs(&self.attrs, t);
        self.path.to_tokens(t);
        let mut inner = TokenStream::new();
        self.fields.to_tokens(&mut inner);
        if self.rest {
            DotDot::default().to_tokens(&mut inner);
        }
        emit_group(Delim::Brace, inner, t);
    }
}
