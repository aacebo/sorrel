use crate::{Spacing, Span};

#[derive(Debug, Clone)]
pub enum Punct {
    #[cfg(nightly)]
    External(proc_macro::Punct),
    Internal {
        ch: char,
        spacing: Spacing,
        span: Span,
    },
}

impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Self {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            return Self::External(proc_macro::Punct::new(ch, spacing.into()));
        }

        Self::Internal {
            ch,
            spacing,
            span: Span::default(),
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            #[cfg(nightly)]
            Self::External(v) => v.as_char(),
            Self::Internal { ch, .. } => *ch,
        }
    }

    pub fn spacing(&self) -> Spacing {
        match self {
            #[cfg(nightly)]
            Self::External(v) => v.spacing().into(),
            Self::Internal { spacing, .. } => *spacing,
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

impl From<proc_macro2::Punct> for Punct {
    fn from(value: proc_macro2::Punct) -> Self {
        Self::Internal {
            ch: value.as_char(),
            spacing: value.spacing().into(),
            span: Span {
                #[cfg(nightly)]
                inner: None,
            },
        }
    }
}

impl From<Punct> for proc_macro2::Punct {
    fn from(value: Punct) -> Self {
        proc_macro2::Punct::new(value.as_char(), value.spacing().into())
    }
}

#[cfg(nightly)]
impl From<proc_macro::Punct> for Punct {
    fn from(value: proc_macro::Punct) -> Self {
        Self::External(value)
    }
}

#[cfg(nightly)]
impl From<Punct> for proc_macro::Punct {
    fn from(value: Punct) -> Self {
        match value {
            Punct::External(v) => v,
            Punct::Internal { ch, spacing, span } => {
                let mut p = proc_macro::Punct::new(ch, spacing.into());
                p.set_span(span.into());
                p
            }
        }
    }
}

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(nightly)]
            Self::External(v) => write!(f, "{}", v),
            Self::Internal { ch, .. } => write!(f, "{}", ch),
        }
    }
}
