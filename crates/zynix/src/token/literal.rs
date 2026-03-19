use crate::Span;

#[derive(Debug, Clone)]
pub enum Literal {
    #[cfg(nightly)]
    External(proc_macro::Literal),
    Internal {
        repr: Box<str>,
        span: Span,
    },
}

macro_rules! lit_constructor {
    ($name:ident, $ty:ty, $fmt:expr) => {
        pub fn $name(value: $ty) -> Self {
            #[cfg(nightly)]
            if proc_macro::is_available() {
                return Self::External(proc_macro::Literal::$name(value));
            }

            Self::Internal {
                repr: format!($fmt, value).into_boxed_str(),
                span: Span::default(),
            }
        }
    };
}

impl Literal {
    pub fn string(value: &str) -> Self {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            return Self::External(proc_macro::Literal::string(value));
        }

        Self::Internal {
            repr: format!("{:?}", value).into_boxed_str(),
            span: Span::default(),
        }
    }

    pub fn character(value: char) -> Self {
        #[cfg(nightly)]
        if proc_macro::is_available() {
            return Self::External(proc_macro::Literal::character(value));
        }

        Self::Internal {
            repr: format!("{:?}", value).into_boxed_str(),
            span: Span::default(),
        }
    }

    lit_constructor!(u8_suffixed, u8, "{}u8");
    lit_constructor!(u16_suffixed, u16, "{}u16");
    lit_constructor!(u32_suffixed, u32, "{}u32");
    lit_constructor!(u64_suffixed, u64, "{}u64");
    lit_constructor!(usize_suffixed, usize, "{}usize");
    lit_constructor!(i8_suffixed, i8, "{}i8");
    lit_constructor!(i16_suffixed, i16, "{}i16");
    lit_constructor!(i32_suffixed, i32, "{}i32");
    lit_constructor!(i64_suffixed, i64, "{}i64");
    lit_constructor!(isize_suffixed, isize, "{}isize");
    lit_constructor!(f32_suffixed, f32, "{}f32");
    lit_constructor!(f64_suffixed, f64, "{}f64");
    lit_constructor!(u8_unsuffixed, u8, "{}");
    lit_constructor!(u16_unsuffixed, u16, "{}");
    lit_constructor!(u32_unsuffixed, u32, "{}");
    lit_constructor!(u64_unsuffixed, u64, "{}");
    lit_constructor!(usize_unsuffixed, usize, "{}");
    lit_constructor!(i8_unsuffixed, i8, "{}");
    lit_constructor!(i16_unsuffixed, i16, "{}");
    lit_constructor!(i32_unsuffixed, i32, "{}");
    lit_constructor!(i64_unsuffixed, i64, "{}");
    lit_constructor!(isize_unsuffixed, isize, "{}");
    lit_constructor!(f32_unsuffixed, f32, "{}");
    lit_constructor!(f64_unsuffixed, f64, "{}");

    pub fn span(&self) -> Span {
        match self {
            #[cfg(nightly)]
            Self::External(v) => v.span().into(),
            Self::Internal { span, .. } => *span,
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            #[cfg(nightly)]
            Self::External(v) => v.set_span(span.into()),
            Self::Internal { span: s, .. } => *s = span,
        }
    }
}

impl From<proc_macro2::Literal> for Literal {
    fn from(value: proc_macro2::Literal) -> Self {
        Self::Internal {
            repr: value.to_string().into_boxed_str(),
            span: Span {
                #[cfg(nightly)]
                inner: None,
            },
        }
    }
}

impl From<Literal> for proc_macro2::Literal {
    fn from(value: Literal) -> Self {
        match value {
            #[cfg(nightly)]
            Literal::External(v) => {
                let repr = v.to_string();
                repr.parse()
                    .unwrap_or_else(|_| proc_macro2::Literal::string(&repr))
            }
            Literal::Internal { repr, .. } => repr
                .parse()
                .unwrap_or_else(|_| proc_macro2::Literal::string(&repr)),
        }
    }
}

#[cfg(nightly)]
impl From<proc_macro::Literal> for Literal {
    fn from(value: proc_macro::Literal) -> Self {
        Self::External(value)
    }
}

#[cfg(nightly)]
impl From<Literal> for proc_macro::Literal {
    fn from(value: Literal) -> Self {
        match value {
            Literal::External(v) => v,
            Literal::Internal { repr, .. } => repr
                .parse()
                .unwrap_or_else(|_| proc_macro::Literal::string(&repr)),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(nightly)]
            Self::External(v) => write!(f, "{}", v),
            Self::Internal { repr, .. } => write!(f, "{}", repr),
        }
    }
}
