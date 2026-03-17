use crate::Span;

#[derive(Debug, Clone)]
pub enum Ident {
    External(proc_macro2::Ident),
    Internal { name: Box<str>, span: Span },
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        Self::Internal {
            name: name.into(),
            span,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::External(v) => v.to_string(),
            Self::Internal { name, .. } => name.to_string(),
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

impl From<proc_macro2::Ident> for Ident {
    fn from(value: proc_macro2::Ident) -> Self {
        Self::External(value)
    }
}

impl From<Ident> for proc_macro2::Ident {
    fn from(value: Ident) -> Self {
        match value {
            Ident::External(v) => v,
            Ident::Internal { name, span } => proc_macro2::Ident::new(&name, span.into()),
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::External(v) => write!(f, "{}", v),
            Self::Internal { name, .. } => write!(f, "{}", name),
        }
    }
}
