use crate::{AsStream, ParseError, Span, Stream, ToStream, report::Level};

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

    #[cfg(nightly)]
    pub fn emit(self) -> Stream {
        proc_macro::Diagnostic::from(self.clone()).emit();
        self.to_stream()
    }

    #[cfg(not(nightly))]
    pub fn emit(self) -> Stream {
        self.to_stream()
    }

    pub fn into_error(self) -> ParseError {
        ParseError::Diagnostic(self)
    }
}

#[cfg(nightly)]
impl From<Diagnostic> for proc_macro::Diagnostic {
    fn from(value: Diagnostic) -> Self {
        let msg = value.message.unwrap_or(String::new());
        let spans: Vec<_> = value
            .spans
            .into_iter()
            .map(|s| proc_macro2::Span::from(s).unwrap())
            .collect();

        let mut new = if spans.is_empty() {
            Self::new(value.level.into(), msg)
        } else {
            Self::spanned(spans, value.level.into(), msg)
        };

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]:", self.level)?;

        if let Some(msg) = &self.message {
            write!(f, ": {}", msg)?;
        }

        for child in &self.children {
            write!(f, "\n  {}", child)?;
        }

        Ok(())
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

impl ToStream for Diagnostic {
    fn to_stream(self) -> Stream {
        self.into_error().to_compile_error()
    }
}

#[cfg(nightly)]
impl proc_macro::ToTokens for Diagnostic {
    fn to_tokens(&self, tokens: &mut proc_macro::TokenStream) {
        let s = self.clone().to_stream().to_string();
        if let Ok(ts) = s.parse::<proc_macro::TokenStream>() {
            tokens.extend(ts);
        }
    }
}
