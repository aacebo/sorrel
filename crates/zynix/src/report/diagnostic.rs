use crate::{ParseError, Span, report::Level};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    level: Level,
    spans: Vec<Span>,
    message: Option<String>,
    children: Vec<Self>,
}

impl Diagnostic {
    pub fn new(level: Level) -> Builder {
        Builder::new().level(level)
    }

    /// the max level of this diagnostic and its children.
    pub fn level(&self) -> Level {
        self.level
    }

    pub fn spans(&self) -> &[Span] {
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

    pub fn into_error(self) -> ParseError {
        ParseError::Diagnostic(self)
    }
}

impl From<Diagnostic> for proc_macro::Diagnostic {
    fn from(value: Diagnostic) -> Self {
        let mut new = Self::new(value.level.into(), value.message.unwrap_or_default());

        for child in value.children {
            let message = child.message.unwrap_or_default();
            let spans: Vec<_> = child
                .spans
                .into_iter()
                .map(proc_macro2::Span::from)
                .map(|span| span.unwrap())
                .collect();

            if child.level.is_error() {
                new = new.span_error(spans, message);
            } else if child.level.is_help() {
                new = new.span_help(spans, message);
            } else if child.level.is_note() {
                new = new.span_note(spans, message);
            } else if child.level.is_warning() {
                new = new.span_warning(spans, message);
            }
        }

        new
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[must_use]
#[derive(Debug, Clone)]
pub struct Builder {
    level: Level,
    spans: Vec<Span>,
    message: Option<String>,
    children: Vec<Diagnostic>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            level: Level::Unknown,
            spans: vec![],
            message: None,
            children: vec![],
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.spans.push(span);
        self
    }

    pub fn spans(mut self, spans: impl Iterator<Item = Span>) -> Self {
        self.spans.extend(spans);
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
