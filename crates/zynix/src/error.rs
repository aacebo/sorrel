use crate::Span;

#[derive(Debug)]
pub enum ParseError {
    Lex(proc_macro2::LexError),
    #[cfg(feature = "report")]
    Diagnostic(crate::report::Diagnostic),
}

impl ParseError {
    pub fn span(&self) -> Option<Span> {
        match self {
            Self::Lex(v) => Some(v.span().into()),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => v.spans().first().cloned(),
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

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lex(v) => write!(f, "{}", v),
            #[cfg(feature = "report")]
            Self::Diagnostic(v) => write!(f, "{}", v),
        }
    }
}
