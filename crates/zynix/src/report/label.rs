use crate::Span;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Label {
    span: Span,
    text: String,
}

impl Label {
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

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.text)
    }
}

impl Span {
    pub fn Label(&self, text: impl std::fmt::Display) -> Label {
        Label::new(*self, text)
    }
}
