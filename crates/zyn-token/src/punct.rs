use crate::{Spacing, Span};

#[derive(Debug, Clone)]
pub struct Punct {
    ch: char,
    spacing: Spacing,
    span: Span,
}

impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Self {
        Self {
            ch,
            spacing,
            span: Span::call_site(),
        }
    }

    pub fn as_char(&self) -> char {
        self.ch
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl From<proc_macro2::Punct> for Punct {
    fn from(value: proc_macro2::Punct) -> Self {
        Self {
            ch: value.as_char(),
            spacing: value.spacing().into(),
            span: value.span().into(),
        }
    }
}

impl From<Punct> for proc_macro2::Punct {
    fn from(value: Punct) -> Self {
        let mut punct = proc_macro2::Punct::new(value.ch, value.spacing.into());
        punct.set_span(value.span.into());
        punct
    }
}

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ch)
    }
}
