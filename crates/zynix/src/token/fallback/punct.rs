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

impl std::fmt::Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ch)
    }
}
