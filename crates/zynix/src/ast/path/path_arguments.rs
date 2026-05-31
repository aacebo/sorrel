use crate::parse::{ParseError, ParseStream};
use crate::token::punct::{Gt, Lt};
use crate::token::{LexError, ToTokens};
use crate::{Parse, TokenStream};

#[doc = "Path segment arguments. `Parenthesized` and full generic-argument parsing are deferred to a later phase; angle-bracketed contents are kept as a raw token stream for now."]
#[derive(Debug, Clone)]
pub enum PathArguments {
    None,
    AngleBracketed(TokenStream),
}

// `AngleBracketed` wraps a raw `TokenStream` — the generic token container, not
// a node — and its delimiters are contextual, so there is no meaningful
// `Parse for TokenStream`. `PathArguments` therefore parses/emits by hand
// rather than delegating to a variant's `Parse`.
impl From<TokenStream> for PathArguments {
    fn from(value: TokenStream) -> Self {
        PathArguments::AngleBracketed(value)
    }
}

impl Parse for PathArguments {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Lt>().is_none() {
            return Ok(PathArguments::None);
        }

        let _ = stream.parse::<Lt>()?;
        let mut inner = TokenStream::new();
        let mut depth = 0usize;

        loop {
            if depth == 0 && stream.peek::<Gt>().is_some() {
                let _ = stream.parse::<Gt>()?;
                break;
            }

            if stream.peek::<Lt>().is_some() {
                depth += 1;
            } else if stream.peek::<Gt>().is_some() {
                depth = depth.saturating_sub(1);
            }

            match stream.advance() {
                Some(tt) => inner.extend_one(tt.clone()),
                None => {
                    return Err(LexError::new(stream.span())
                        .message("unterminated `<...>` in path arguments")
                        .into());
                }
            }
        }

        Ok(PathArguments::AngleBracketed(inner))
    }
}

impl ToTokens for PathArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PathArguments::None => {}
            PathArguments::AngleBracketed(inner) => {
                Lt::default().to_tokens(tokens);
                inner.to_tokens(tokens);
                Gt::default().to_tokens(tokens);
            }
        }
    }
}
