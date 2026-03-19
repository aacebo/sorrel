use crate::{Delim, DelimSpan, TokenStream};

#[derive(Debug, Clone)]
pub struct Group {
    delim: Delim,
    span: DelimSpan,
    tokens: TokenStream,
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

impl From<proc_macro2::Group> for Group {
    fn from(value: proc_macro2::Group) -> Self {
        Self::new(value.delimiter().into(), value.stream().into())
    }
}

impl From<Group> for proc_macro2::Group {
    fn from(value: Group) -> Self {
        let stream: proc_macro2::TokenStream = value.tokens.into();
        proc_macro2::Group::new(value.delim.into(), stream)
    }
}

#[cfg(nightly)]
impl From<proc_macro::Group> for Group {
    fn from(value: proc_macro::Group) -> Self {
        Self::new(value.delimiter().into(), value.stream().into())
    }
}

#[cfg(nightly)]
impl From<Group> for proc_macro::Group {
    fn from(value: Group) -> Self {
        proc_macro::Group::new(value.delim.into(), value.tokens.into())
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.tokens)
    }
}
