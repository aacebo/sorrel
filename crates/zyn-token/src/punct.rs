use crate::{Spacing, Span};

#[derive(Debug, Clone)]
pub enum Punct {
    External(proc_macro2::Punct),
    Internal {
        ch: char,
        spacing: Spacing,
        span: Span,
    },
}

impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Self {
        Self::Internal {
            ch,
            spacing,
            span: Span::call_site(),
        }
    }

    pub fn as_char(&self) -> char {
        match self {
            Self::External(v) => v.as_char(),
            Self::Internal { ch, .. } => *ch,
        }
    }

    pub fn spacing(&self) -> Spacing {
        match self {
            Self::External(v) => v.spacing().into(),
            Self::Internal { spacing, .. } => *spacing,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::External(v) => v.span().into(),
            Self::Internal { span, .. } => *span,
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::External(v) => v.set_span(span.into()),
            Self::Internal { span: s, .. } => *s = span,
        }
    }
}

impl From<proc_macro2::Punct> for Punct {
    fn from(value: proc_macro2::Punct) -> Self {
        Self::External(value)
    }
}

impl From<Punct> for proc_macro2::Punct {
    fn from(value: Punct) -> Self {
        match value {
            Punct::External(v) => v,
            Punct::Internal { ch, spacing, span } => {
                let mut p = proc_macro2::Punct::new(ch, spacing.into());
                p.set_span(span.into());
                p
            }
        }
    }
}

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::External(v) => write!(f, "{}", v),
            Self::Internal { ch, .. } => write!(f, "{}", ch),
        }
    }
}
