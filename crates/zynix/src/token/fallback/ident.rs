use std::borrow::Cow;

use crate::Span;

#[derive(Debug, Clone)]
pub struct Ident {
    pub(crate) name: Box<str>,
    pub(crate) span: Span,
}

impl Ident {
    pub fn new(name: &str, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }

    pub fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.name.as_ref())
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
