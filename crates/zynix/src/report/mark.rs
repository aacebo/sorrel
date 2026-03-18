use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mark {
    span: Span,
    text: String,
}

impl Mark {
    pub fn new(span: Span, text: impl std::fmt::Display) -> Self {
        Self {
            span,
            text: text.to_string(),
        }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.text)
    }
}

impl Span {
    pub fn mark(&self, text: impl std::fmt::Display) -> Mark {
        Mark::new(*self, text)
    }
}
