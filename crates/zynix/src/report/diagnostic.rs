use crate::{Span, report::Level};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    span: Span,
    level: Level,
    message: Option<String>,
    children: Vec<Self>,
}

impl Diagnostic {
    pub fn new(span: Span, level: Level) -> Builder {
        Builder::new().span(span).level(level)
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn level(&self) -> Level {
        self.level
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
    span: Option<Span>,
    level: Level,
    message: Option<String>,
    children: Vec<Diagnostic>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            span: None,
            level: Level::Unknown,
            message: None,
            children: vec![],
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = Some(span);
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

    pub fn build(mut self) -> Diagnostic {
        self.children.sort();

        let span = self.span.unwrap_or_else(|| {
            let first = self
                .children
                .first()
                .map(|v| v.span())
                .unwrap_or(Span::call_site());

            let last = self
                .children
                .last()
                .map(|v| v.span())
                .unwrap_or(Span::call_site());

            Span::range(first, last)
        });

        let mut level = self.level;

        for child in &self.children {
            let clevel = child.level();

            if clevel > level {
                level = clevel;
            }
        }

        Diagnostic {
            span,
            level,
            message: self.message,
            children: self.children,
        }
    }
}

impl Eq for Diagnostic {}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.span == other.span
    }
}

impl Ord for Diagnostic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.span.cmp(&other.span)
    }
}

impl PartialOrd for Diagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
