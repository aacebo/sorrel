use std::borrow::Cow;

use super::fallback;
use crate::Span;

#[derive(Debug, Clone)]
pub enum Ident {
    Compiler(proc_macro::Ident),
    Fallback(fallback::Ident),
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Ident::new(name, span.into()))
        } else {
            Self::Fallback(fallback::Ident::new(name, span))
        }
    }

    pub fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Compiler(v) => Cow::Owned(v.to_string()),
            Self::Fallback(v) => v.name(),
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

impl From<proc_macro2::Ident> for Ident {
    fn from(value: proc_macro2::Ident) -> Self {
        Self::Fallback(fallback::Ident::new(&value.to_string(), Span::default()))
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(value: Ident) -> Self {
        match value {
            Ident::Compiler(v) => {
                proc_macro2::Ident::new(&v.to_string(), proc_macro2::Span::call_site())
            }
            Ident::Fallback(v) => proc_macro2::Ident::new(&v.name, proc_macro2::Span::call_site()),
        }
    }
}

impl From<proc_macro::Ident> for Ident {
    fn from(value: proc_macro::Ident) -> Self {
        Self::Compiler(value)
    }
}

impl From<Ident> for proc_macro::Ident {
    fn from(value: Ident) -> Self {
        match value {
            Ident::Compiler(v) => v,
            Ident::Fallback(v) => proc_macro::Ident::new(&v.name, v.span.into()),
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(v) => write!(f, "{}", v),
            Self::Fallback(v) => write!(f, "{}", v),
        }
    }
}
