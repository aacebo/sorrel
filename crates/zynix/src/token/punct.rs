use super::fallback;
use crate::{Spacing, Span};

#[derive(Debug, Clone)]
pub enum Punct {
    Compiler(proc_macro::Punct),
    Fallback(fallback::Punct),
}

impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Punct::new(ch, spacing.into()))
        } else {
            Self::Fallback(fallback::Punct::new(ch, spacing))
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Self::Compiler(v) => v.as_char(),
            Self::Fallback(v) => v.as_char(),
        }
    }

    pub fn spacing(&self) -> Spacing {
        match self {
            Self::Compiler(v) => v.spacing().into(),
            Self::Fallback(v) => v.spacing(),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Compiler(v) => v.span().into(),
            Self::Fallback(v) => v.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Compiler(v) => v.set_span(span.into()),
            Self::Fallback(v) => v.set_span(span),
        }
    }
}

impl From<proc_macro2::Punct> for Punct {
    fn from(value: proc_macro2::Punct) -> Self {
        Self::Fallback(fallback::Punct {
            ch: value.as_char(),
            spacing: value.spacing().into(),
            span: Span::default(),
        })
    }
}

impl From<Punct> for proc_macro2::Punct {
    fn from(value: Punct) -> Self {
        proc_macro2::Punct::new(value.as_char(), value.spacing().into())
    }
}

impl From<proc_macro::Punct> for Punct {
    fn from(value: proc_macro::Punct) -> Self {
        Self::Compiler(value)
    }
}

impl From<Punct> for proc_macro::Punct {
    fn from(value: Punct) -> Self {
        match value {
            Punct::Compiler(v) => v,
            Punct::Fallback(v) => {
                let mut p = proc_macro::Punct::new(v.ch, v.spacing.into());
                p.set_span(v.span.into());
                p
            }
        }
    }
}

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(v) => write!(f, "{}", v),
            Self::Fallback(v) => write!(f, "{}", v),
        }
    }
}
