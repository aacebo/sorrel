use crate::{Span, SpanSet, Spanner};

#[derive(Debug)]
pub enum ParseError {
    Lex(proc_macro2::LexError),
    #[cfg(feature = "report")]
    Diagnostic(crate::report::Diagnostic),
}

impl ParseError {
    pub fn span(&self) -> Span {
        match self {
            Self::Lex(v) => v.span().into(),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => v.span().span(),
        }
    }
}

impl From<proc_macro2::LexError> for ParseError {
    fn from(value: proc_macro2::LexError) -> Self {
        Self::Lex(value)
    }
}

#[cfg(feature = "report")]
impl From<crate::report::Diagnostic> for ParseError {
    fn from(value: crate::report::Diagnostic) -> Self {
        Self::Diagnostic(value)
    }
}

impl Spanner for ParseError {
    fn span(&self) -> Span {
        self.span()
    }

    fn into_spans(self) -> SpanSet
    where
        Self: Sized,
    {
        match self {
            Self::Lex(v) => SpanSet::new(v.span().into()),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => v.into_spans(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lex(v) => write!(f, "{}", v),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => write!(f, "{}", v),
        }
    }
}
