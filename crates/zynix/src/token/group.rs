use super::fallback;
use crate::{Delim, DelimSpan, TokenStream};

#[derive(Debug, Clone)]
pub enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
}

impl Group {
    pub fn new(delim: Delim, stream: TokenStream) -> Self {
        match stream {
            TokenStream::Compiler(v) => Self::Compiler(proc_macro::Group::new(delim.into(), v)),
            TokenStream::Fallback(v) => Self::Fallback(fallback::Group::new(delim, v)),
        }
    }

    pub fn delim(&self) -> Delim {
        match self {
            Self::Compiler(v) => v.delimiter().into(),
            Self::Fallback(v) => v.delim(),
        }
    }

    pub fn span(&self) -> DelimSpan {
        match self {
            Self::Compiler(v) => DelimSpan::new(v.span_open().into(), v.span_close().into()),
            Self::Fallback(v) => v.span(),
        }
    }

    pub fn stream(&self) -> TokenStream {
        match self {
            Self::Compiler(v) => v.stream().into(),
            Self::Fallback(v) => v.stream().into(),
        }
    }
}

impl From<proc_macro::Group> for Group {
    fn from(value: proc_macro::Group) -> Self {
        Self::Compiler(value)
    }
}

impl From<Group> for proc_macro::Group {
    fn from(value: Group) -> Self {
        match value {
            Group::Compiler(v) => v,
            Group::Fallback(v) => {
                let span = v.span.span();
                let mut group = proc_macro::Group::new(v.delim.into(), v.tokens.into());
                group.set_span(span.into());
                group
            }
        }
    }
}

impl From<fallback::Group> for Group {
    fn from(value: fallback::Group) -> Self {
        Self::Fallback(value)
    }
}

impl From<Group> for fallback::Group {
    fn from(value: Group) -> Self {
        match value {
            Group::Compiler(v) => fallback::Group::new(v.delimiter().into(), v.stream().into()),
            Group::Fallback(v) => v,
        }
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(v) => write!(f, "{}", v),
            Self::Fallback(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(nightly)]
impl proc_macro::ToTokens for Group {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        use crate::Token;

        tokens.extend_one(Token::from(self.clone()).to_tree());
    }
}
