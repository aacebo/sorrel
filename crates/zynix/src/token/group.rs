use super::fallback;
use crate::{Delim, DelimSpan, TokenStream};

#[derive(Debug, Clone)]
pub enum Group {
    Compiler(proc_macro::Group),
    Fallback(fallback::Group),
}

impl Group {
    pub fn new(delim: Delim, stream: TokenStream) -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Group::new(delim.into(), stream.into()))
        } else {
            Self::Fallback(fallback::Group::new(delim, stream))
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
            Self::Compiler(v) => {
                let span = v.span().into();
                DelimSpan::new(span, span)
            }
            Self::Fallback(v) => v.span(),
        }
    }

    pub fn as_tokens(&self) -> &TokenStream {
        match self {
            Self::Compiler(_) => {
                panic!("cannot borrow tokens from compiler group; normalize first")
            }
            Self::Fallback(v) => v.as_tokens(),
        }
    }
}

impl From<proc_macro2::Group> for Group {
    fn from(value: proc_macro2::Group) -> Self {
        Self::Fallback(fallback::Group::new(
            value.delimiter().into(),
            value.stream().into(),
        ))
    }
}

impl From<Group> for proc_macro2::Group {
    fn from(value: Group) -> Self {
        match value {
            Group::Compiler(v) => {
                let delim: Delim = v.delimiter().into();
                let stream: proc_macro2::TokenStream = proc_macro2::TokenStream::from(v.stream());
                proc_macro2::Group::new(delim.into(), stream)
            }
            Group::Fallback(v) => {
                let stream: proc_macro2::TokenStream = v.tokens.into();
                proc_macro2::Group::new(v.delim.into(), stream)
            }
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
            Group::Fallback(v) => proc_macro::Group::new(v.delim.into(), v.tokens.into()),
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
