mod delim;
pub(crate) mod fallback;
mod range;

pub use delim::*;
pub use range::*;

#[derive(Debug, Copy, Clone)]
pub enum Span {
    Compiler(proc_macro::Span),
    Fallback(fallback::Span),
}

impl Span {
    pub fn call_site() -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Span::call_site())
        } else {
            Self::Fallback(fallback::Span::call_site())
        }
    }

    pub fn mixed_site() -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Span::mixed_site())
        } else {
            Self::Fallback(fallback::Span::mixed_site())
        }
    }

    pub fn def_site() -> Self {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            return Self::Compiler(proc_macro::Span::def_site());
        }

        Self::Fallback(fallback::Span::def_site())
    }

    pub fn range(from: Self, to: Self) -> Self {
        from.join(to)
    }

    pub fn join(&self, other: Self) -> Self {
        #[cfg(nightly)]
        if let (Self::Compiler(a), Self::Compiler(b)) = (self, other) {
            if let Some(joined) = a.join(b) {
                return Self::Compiler(joined);
            }
        }

        other
    }
}

impl Default for Span {
    fn default() -> Self {
        if proc_macro::is_available() {
            Self::call_site()
        } else {
            Self::Fallback(fallback::Span::call_site())
        }
    }
}

impl Eq for Span {}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        #[cfg(nightly)]
        {
            match (self, other) {
                (Self::Compiler(a), Self::Compiler(b)) => {
                    a.line() == b.line()
                        && a.column() == b.column()
                        && a.end().line() == b.end().line()
                        && a.end().column() == b.end().column()
                }
                (Self::Fallback(_), Self::Fallback(_)) => true,
                _ => false,
            }
        }
        #[cfg(not(nightly))]
        true
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        #[cfg(nightly)]
        {
            match (self, other) {
                (Self::Compiler(a), Self::Compiler(b)) => match a.line().cmp(&b.line()) {
                    std::cmp::Ordering::Equal => match a.column().cmp(&b.column()) {
                        std::cmp::Ordering::Equal => match a.end().line().cmp(&b.end().line()) {
                            std::cmp::Ordering::Equal => a.end().column().cmp(&b.end().column()),
                            ord => ord,
                        },
                        ord => ord,
                    },
                    ord => ord,
                },
                (Self::Fallback(_), Self::Fallback(_)) => std::cmp::Ordering::Equal,
                (Self::Fallback(_), Self::Compiler(_)) => std::cmp::Ordering::Less,
                (Self::Compiler(_), Self::Fallback(_)) => std::cmp::Ordering::Greater,
            }
        }
        #[cfg(not(nightly))]
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for Span {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        #[cfg(nightly)]
        if let Self::Compiler(s) = self {
            s.line().hash(state);
            s.column().hash(state);
            s.end().line().hash(state);
            s.end().column().hash(state);
        }
    }
}

impl From<proc_macro::Span> for Span {
    fn from(value: proc_macro::Span) -> Self {
        Self::Compiler(value)
    }
}

impl From<fallback::Span> for Span {
    fn from(value: fallback::Span) -> Self {
        Self::Fallback(value)
    }
}

impl From<Span> for proc_macro::Span {
    fn from(value: Span) -> Self {
        match value {
            Span::Compiler(s) => s,
            Span::Fallback(_) => proc_macro::Span::call_site(),
        }
    }
}

#[cfg(nightly)]
impl proc_macro::MultiSpan for Span {
    fn into_spans(self) -> Vec<proc_macro::Span> {
        match self {
            Self::Compiler(s) => vec![s],
            Self::Fallback(_) => vec![proc_macro::Span::call_site()],
        }
    }
}
