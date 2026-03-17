use std::fmt;

use crate::Span;

#[derive(Clone)]
pub struct SpanError {
    messages: Vec<(Span, String)>,
}

impl SpanError {
    pub fn new(span: Span, message: impl fmt::Display) -> Self {
        Self {
            messages: vec![(span, message.to_string())],
        }
    }

    pub fn call_site(message: impl fmt::Display) -> Self {
        Self::new(Span::call_site(), message)
    }

    pub fn span(&self) -> Span {
        self.messages[0].0
    }

    pub fn join(&mut self, other: Self) -> &mut Self {
        self.messages.extend(other.messages);
        self
    }

    pub fn add(mut self, span: Span, message: impl Into<String>) -> Self {
        self.messages.push((span, message.into()));
        self
    }
}

impl From<proc_macro2::LexError> for SpanError {
    fn from(e: proc_macro2::LexError) -> Self {
        Self::new(e.span().into(), e)
    }
}

impl fmt::Display for SpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (_, msg)) in self.messages.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }

            f.write_str(msg)?;
        }

        Ok(())
    }
}

impl fmt::Debug for SpanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for SpanError {}
