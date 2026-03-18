mod diagnostic;
mod level;
mod mark;

pub use diagnostic::*;
pub use level::*;
pub use mark::*;

impl crate::Span {
    pub fn error(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new(*self, Level::Error)
            .message(message)
            .build()
    }

    pub fn warn(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new(*self, Level::Warning)
            .message(message)
            .build()
    }

    pub fn note(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new(*self, Level::Note).message(message).build()
    }

    pub fn help(&self, message: impl std::fmt::Display) -> Diagnostic {
        Diagnostic::new(*self, Level::Help).message(message).build()
    }
}
