use crate::ast::{Attribute, DelimiterKind, Path};
use crate::parse::{ParseError, ParseStream};
use crate::token::punct::Not;
use crate::token::{Delim, Group, LexError, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A macro invocation (`path!(...)`, `path![...]`, `path!{...}`)."]
#[derive(Debug, Clone)]
pub struct MacroCall {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub path: Path,
    pub delimiter: DelimiterKind,
    pub tokens: TokenStream,
}

impl Parse for MacroCall {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec()?;
        let path = stream.parse::<Path>()?;
        let _ = stream.parse::<Not>()?;

        let (delimiter, tokens) = match stream.curr() {
            Some(TokenTree::Group(g)) => {
                let delim = match g.delim() {
                    Delim::Paren => DelimiterKind::Paren,
                    Delim::Bracket => DelimiterKind::Bracket,
                    Delim::Brace => DelimiterKind::Brace,
                    Delim::None => {
                        return Err(LexError::new(stream.span())
                            .message("expected macro delimiter")
                            .into());
                    }
                };
                let tokens = g.stream();
                stream.advance();
                (delim, tokens)
            }
            _ => {
                return Err(LexError::new(stream.span())
                    .message("expected macro delimiter")
                    .into());
            }
        };

        Ok(Self {
            span: Span::default(),
            attrs,
            path,
            delimiter,
            tokens,
        })
    }
}

impl ToTokens for MacroCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.attrs {
            attr.to_tokens(tokens);
        }
        self.path.to_tokens(tokens);
        Not::default().to_tokens(tokens);
        let delim = match self.delimiter {
            DelimiterKind::Paren => Delim::Paren,
            DelimiterKind::Bracket => Delim::Bracket,
            DelimiterKind::Brace => Delim::Brace,
        };
        tokens.extend_one(TokenTree::Group(Group::new(delim, self.tokens.clone())));
    }
}
