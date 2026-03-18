mod diagnostic;
mod level;

pub use diagnostic::*;
pub use level::*;

impl crate::Span {
    pub fn error(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new(Level::Error)
            .span(*self)
            .message(message)
            .build()
    }

    pub fn warn(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new(Level::Warning)
            .span(*self)
            .message(message)
            .build()
    }

    pub fn note(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new(Level::Note)
            .span(*self)
            .message(message)
            .build()
    }

    pub fn help(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new(Level::Help)
            .span(*self)
            .message(message)
            .build()
    }
}
