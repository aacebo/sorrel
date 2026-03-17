#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Error,
    Warning,
    Note,
    Help,
    Unknown,
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Note => "note",
            Self::Help => "help",
            Self::Unknown => "??",
        }
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(nightly)]
impl From<Level> for proc_macro::Level {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => proc_macro::Level::Error,
            Level::Warning => proc_macro::Level::Warning,
            Level::Note => proc_macro::Level::Note,
            Level::Help => proc_macro::Level::Help,
            Level::Unknown => unreachable!(),
        }
    }
}

#[cfg(nightly)]
impl From<proc_macro::Level> for Level {
    fn from(value: proc_macro::Level) -> Self {
        match value {
            proc_macro::Level::Error => Self::Error,
            proc_macro::Level::Warning => Self::Warning,
            proc_macro::Level::Note => Self::Note,
            proc_macro::Level::Help => Self::Help,
            _ => Self::Unknown,
        }
    }
}
