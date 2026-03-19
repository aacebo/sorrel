use std::borrow::Cow;

use crate::Span;

#[derive(Debug, Clone)]
pub enum Ident {
    #[cfg(nightly)]
    External(proc_macro::Ident),
    Internal {
        name: Box<str>,
        span: Span,
    },
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            return Self::External(proc_macro::Ident::new(name, span.into()));
        }

        Self::Internal {
            name: name.into(),
            span,
        }
    }

    pub fn name(&self) -> Cow<'_, str> {
        match self {
            #[cfg(nightly)]
            Self::External(v) => Cow::Owned(v.to_string()),
            Self::Internal { name, .. } => Cow::Borrowed(name.as_ref()),
        }
    }

    pub fn span(&self) -> Span {
        match self {
            #[cfg(nightly)]
            Self::External(v) => v.span().into(),
            Self::Internal { span, .. } => *span,
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            #[cfg(nightly)]
            Self::External(v) => v.set_span(span.into()),
            Self::Internal { span: s, .. } => *s = span,
        }
    }
}

impl From<proc_macro2::Ident> for Ident {
    fn from(value: proc_macro2::Ident) -> Self {
        Self::Internal {
            name: value.to_string().into_boxed_str(),
            span: Span {
                #[cfg(nightly)]
                inner: None,
            },
        }
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(value: Ident) -> Self {
        match value {
            #[cfg(nightly)]
            Ident::External(v) => {
                proc_macro2::Ident::new(&v.to_string(), proc_macro2::Span::call_site())
            }
            Ident::Internal { name, .. } => {
                proc_macro2::Ident::new(&name, proc_macro2::Span::call_site())
            }
        }
    }
}

#[cfg(nightly)]
impl From<proc_macro::Ident> for Ident {
    fn from(value: proc_macro::Ident) -> Self {
        Self::External(value)
    }
}

#[cfg(nightly)]
impl From<Ident> for proc_macro::Ident {
    fn from(value: Ident) -> Self {
        match value {
            Ident::External(v) => v,
            Ident::Internal { name, span } => proc_macro::Ident::new(&name, span.into()),
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(nightly)]
            Self::External(v) => write!(f, "{}", v),
            Self::Internal { name, .. } => write!(f, "{}", name),
        }
    }
}
