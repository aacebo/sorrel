use crate::{Delim, DelimSpan, TokenStream};

#[derive(Debug, Clone)]
pub struct Group {
    pub(crate) delim: Delim,
    pub(crate) span: DelimSpan,
    pub(crate) tokens: TokenStream,
}

impl Group {
    pub fn new(delim: Delim, mut stream: TokenStream) -> Self {
        Self {
            delim,
            span: stream.delim(),
            tokens: stream,
        }
    }

    pub fn delim(&self) -> Delim {
        self.delim
    }

    pub fn span(&self) -> DelimSpan {
        self.span
    }

    pub fn as_tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.tokens)
    }
}
