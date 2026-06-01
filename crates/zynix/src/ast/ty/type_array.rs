use super::Type;
use crate::ast::Expr;
use crate::token::ToTokens;
use crate::{Span, TokenStream};

#[doc = "A fixed-size array type (`[T; N]`)."]
#[derive(Debug, Clone)]
pub struct TypeArray {
    pub span: Span,
    pub elem: Box<Type>,
    pub len: Expr,
}

impl ToTokens for TypeArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut inner = TokenStream::new();
        self.elem.to_tokens(&mut inner);
        crate::token::punct::Semi::default().to_tokens(&mut inner);
        self.len.to_tokens(&mut inner);
        tokens.extend_one(crate::TokenTree::Group(crate::token::Group::new(
            crate::token::Delim::Bracket,
            inner,
        )));
    }
}
