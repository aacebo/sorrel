mod delim;
mod range;

pub use delim::*;
pub use range::*;

#[derive(Debug, Copy, Clone)]
pub struct Span {
    #[cfg(nightly)]
    pub(crate) inner: Option<proc_macro::Span>,
}

impl Span {
    pub fn call_site() -> Self {
        Self {
            #[cfg(nightly)]
            inner: Some(proc_macro::Span::call_site()),
        }
    }

    pub fn mixed_site() -> Self {
        Self {
            #[cfg(nightly)]
            inner: Some(proc_macro::Span::mixed_site()),
        }
    }

    pub fn range(from: Self, to: Self) -> Self {
        from.join(to)
    }

    pub fn join(&self, other: Self) -> Self {
        #[cfg(nightly)]
        if let (Some(a), Some(b)) = (self.inner, other.inner) {
            if let Some(joined) = a.join(b) {
                return Self {
                    inner: Some(joined),
                };
            }
        }

        other
    }
}

impl Default for Span {
    fn default() -> Self {
        if cfg!(nightly) && proc_macro::is_available() {
            Self::call_site()
        } else {
            Self {
                #[cfg(nightly)]
                inner: None,
            }
        }
    }
}

impl Eq for Span {}

impl PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        #[cfg(nightly)]
        {
            match (self.inner, other.inner) {
                (None, None) => true,
                (Some(a), Some(b)) => {
                    a.line() == b.line()
                        && a.column() == b.column()
                        && a.end().line() == b.end().line()
                        && a.end().column() == b.end().column()
                }
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
            match (self.inner, other.inner) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Less,
                (Some(_), None) => std::cmp::Ordering::Greater,
                (Some(a), Some(b)) => match a.line().cmp(&b.line()) {
                    std::cmp::Ordering::Equal => match a.column().cmp(&b.column()) {
                        std::cmp::Ordering::Equal => match a.end().line().cmp(&b.end().line()) {
                            std::cmp::Ordering::Equal => a.end().column().cmp(&b.end().column()),
                            ord => ord,
                        },
                        ord => ord,
                    },
                    ord => ord,
                },
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
        if let Some(s) = self.inner {
            s.line().hash(state);
            s.column().hash(state);
            s.end().line().hash(state);
            s.end().column().hash(state);
        }
    }
}

#[cfg(nightly)]
impl From<proc_macro::Span> for Span {
    fn from(value: proc_macro::Span) -> Self {
        Self { inner: Some(value) }
    }
}

#[cfg(nightly)]
impl From<Span> for proc_macro::Span {
    fn from(value: Span) -> Self {
        value.inner.unwrap_or_else(proc_macro::Span::call_site)
    }
}

#[cfg(nightly)]
impl proc_macro::MultiSpan for Span {
    fn into_spans(self) -> Vec<proc_macro::Span> {
        match self.inner {
            Some(s) => vec![s],
            None => vec![proc_macro::Span::call_site()],
        }
    }
}
