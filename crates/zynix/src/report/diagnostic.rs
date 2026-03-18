use crate::{SpanSet, Spanner, report::Level};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    level: Level,
    spans: SpanSet,
    message: Option<String>,
    children: Vec<Self>,
}

impl Diagnostic {
    pub fn new(span: impl Spanner, level: Level) -> Builder {
        Builder::new().span(span).level(level)
    }

    /// the max level of this diagnostic and its children.
    pub fn level(&self) -> Level {
        self.level
    }

    pub fn span(&self) -> &SpanSet {
        &self.spans
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_ref().map(|m| m.as_str())
    }

    pub fn children(&self) -> &[Self] {
        &self.children
    }

    pub fn emit(self) {
        if proc_macro::is_available() {
            proc_macro::Diagnostic::from(self).emit();
        } else {
            todo!()
        }
    }
}

impl From<Diagnostic> for proc_macro::Diagnostic {
    fn from(value: Diagnostic) -> Self {
        let mut new = Self::new(value.level.into(), value.message.unwrap_or_default());

        for child in value.children {
            let message = child.message().unwrap_or_default();

            if child.level.is_error() {
                new = new.span_error(child.span(), message);
            } else if child.level.is_help() {
                new = new.span_help(child.span(), message);
            } else if child.level.is_note() {
                new = new.span_note(child.span(), message);
            } else if child.level.is_warning() {
                new = new.span_warning(child.span(), message);
            }
        }

        new
    }
}

#[must_use]
#[derive(Debug, Clone)]
pub struct Builder {
    level: Level,
    spans: SpanSet,
    message: Option<String>,
    children: Vec<Diagnostic>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            level: Level::Unknown,
            spans: SpanSet::default(),
            message: None,
            children: vec![],
        }
    }

    pub fn span(mut self, span: impl Spanner) -> Self {
        self.spans = span.into_spans();
        self
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn message(mut self, message: impl std::fmt::Display) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub fn add(mut self, child: Diagnostic) -> Self {
        self.children.push(child);
        self
    }

    pub fn build(self) -> Diagnostic {
        let mut level = self.level;

        for child in &self.children {
            let clevel = child.level();

            if clevel > level {
                level = clevel;
            }
        }

        Diagnostic {
            spans: self.spans,
            level,
            message: self.message,
            children: self.children,
        }
    }
}

impl Eq for Diagnostic {}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.spans == other.spans
    }
}
