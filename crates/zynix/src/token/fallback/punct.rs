use crate::{Spacing, Span};

#[derive(Debug, Clone)]
pub struct Punct {
    pub(crate) ch: char,
    pub(crate) spacing: Spacing,
    pub(crate) span: Span,
}

impl Punct {
    pub fn new(ch: char, spacing: Spacing) -> Self {
        Self {
            ch,
            spacing,
            span: Span::default(),
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

impl From<proc_macro::Punct> for Punct {
    fn from(value: proc_macro::Punct) -> Self {
        Self {
            ch: value.as_char(),
            spacing: value.spacing().into(),
            span: value.span().into(),
        }
    }
}

impl From<Punct> for proc_macro::Punct {
    fn from(value: Punct) -> Self {
        let mut p = proc_macro::Punct::new(value.ch, value.spacing.into());
        p.set_span(value.span.into());
        p
    }
}

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ch)
    }
}
