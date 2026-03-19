use super::fallback;
use crate::Span;

#[derive(Debug, Clone)]
pub enum Literal {
    Compiler(proc_macro::Literal),
    Fallback(fallback::Literal),
}

macro_rules! lit_constructor {
    ($name:ident, $ty:ty) => {
        pub fn $name(value: $ty) -> Self {
            if proc_macro::is_available() {
                Self::Compiler(proc_macro::Literal::$name(value))
            } else {
                Self::Fallback(fallback::Literal::$name(value))
            }
        }
    };
}

impl Literal {
    pub fn string(value: &str) -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Literal::string(value))
        } else {
            Self::Fallback(fallback::Literal::string(value))
        }
    }

    pub fn character(value: char) -> Self {
        if proc_macro::is_available() {
            Self::Compiler(proc_macro::Literal::character(value))
        } else {
            Self::Fallback(fallback::Literal::character(value))
        }
    }

    lit_constructor!(u8_suffixed, u8);
    lit_constructor!(u16_suffixed, u16);
    lit_constructor!(u32_suffixed, u32);
    lit_constructor!(u64_suffixed, u64);
    lit_constructor!(usize_suffixed, usize);
    lit_constructor!(i8_suffixed, i8);
    lit_constructor!(i16_suffixed, i16);
    lit_constructor!(i32_suffixed, i32);
    lit_constructor!(i64_suffixed, i64);
    lit_constructor!(isize_suffixed, isize);
    lit_constructor!(f32_suffixed, f32);
    lit_constructor!(f64_suffixed, f64);
    lit_constructor!(u8_unsuffixed, u8);
    lit_constructor!(u16_unsuffixed, u16);
    lit_constructor!(u32_unsuffixed, u32);
    lit_constructor!(u64_unsuffixed, u64);
    lit_constructor!(usize_unsuffixed, usize);
    lit_constructor!(i8_unsuffixed, i8);
    lit_constructor!(i16_unsuffixed, i16);
    lit_constructor!(i32_unsuffixed, i32);
    lit_constructor!(i64_unsuffixed, i64);
    lit_constructor!(isize_unsuffixed, isize);
    lit_constructor!(f32_unsuffixed, f32);
    lit_constructor!(f64_unsuffixed, f64);

    pub fn span(&self) -> Span {
        match self {
            Self::Compiler(v) => v.span().into(),
            Self::Fallback(v) => v.span(),
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::Compiler(v) => v.set_span(span.into()),
            Self::Fallback(v) => v.set_span(span),
        }
    }
}

impl From<proc_macro::Literal> for Literal {
    fn from(value: proc_macro::Literal) -> Self {
        Self::Compiler(value)
    }
}

impl From<Literal> for proc_macro::Literal {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Compiler(v) => v,
            Literal::Fallback(v) => v
                .repr
                .parse()
                .unwrap_or_else(|_| proc_macro::Literal::string(&v.repr)),
        }
    }
}

impl From<fallback::Literal> for Literal {
    fn from(value: fallback::Literal) -> Self {
        Self::Fallback(value)
    }
}

impl From<Literal> for fallback::Literal {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Compiler(v) => fallback::Literal {
                repr: v.to_string().into_boxed_str(),
                span: v.span().into(),
            },
            Literal::Fallback(v) => v,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiler(v) => write!(f, "{}", v),
            Self::Fallback(v) => write!(f, "{}", v),
        }
    }
}
