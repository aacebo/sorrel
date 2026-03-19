use crate::{Delim, DelimSpan, TokenStream};

#[derive(Debug, Clone)]
pub struct Group {
    delim: Delim,
    span: DelimSpan,
    tokens: TokenStream,
}

impl Group {
    pub fn new(delim: Delim, stream: TokenStream) -> Self {
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

impl From<proc_macro2::Group> for Group {
    fn from(value: proc_macro2::Group) -> Self {
        Self {
            delim: value.delimiter().into(),
            span: value.delim_span().into(),
            tokens: value.stream().into(),
        }
    }
}

impl From<Group> for proc_macro2::Group {
    fn from(value: Group) -> Self {
        Self::new(value.delim().into(), value.tokens.into())
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.tokens)
    }
}
