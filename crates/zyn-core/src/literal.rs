use crate::Span;

#[derive(Debug, Clone)]
pub enum Literal {
    External(proc_macro2::Literal),
    Internal { repr: Box<str>, span: Span },
}

impl Literal {
    pub fn string(value: &str) -> Self {
        Self::External(proc_macro2::Literal::string(value))
    }

    pub fn character(value: char) -> Self {
        Self::External(proc_macro2::Literal::character(value))
    }

    pub fn u8_suffixed(value: u8) -> Self {
        Self::External(proc_macro2::Literal::u8_suffixed(value))
    }

    pub fn u16_suffixed(value: u16) -> Self {
        Self::External(proc_macro2::Literal::u16_suffixed(value))
    }

    pub fn u32_suffixed(value: u32) -> Self {
        Self::External(proc_macro2::Literal::u32_suffixed(value))
    }

    pub fn u64_suffixed(value: u64) -> Self {
        Self::External(proc_macro2::Literal::u64_suffixed(value))
    }

    pub fn usize_suffixed(value: usize) -> Self {
        Self::External(proc_macro2::Literal::usize_suffixed(value))
    }

    pub fn i8_suffixed(value: i8) -> Self {
        Self::External(proc_macro2::Literal::i8_suffixed(value))
    }

    pub fn i16_suffixed(value: i16) -> Self {
        Self::External(proc_macro2::Literal::i16_suffixed(value))
    }

    pub fn i32_suffixed(value: i32) -> Self {
        Self::External(proc_macro2::Literal::i32_suffixed(value))
    }

    pub fn i64_suffixed(value: i64) -> Self {
        Self::External(proc_macro2::Literal::i64_suffixed(value))
    }

    pub fn isize_suffixed(value: isize) -> Self {
        Self::External(proc_macro2::Literal::isize_suffixed(value))
    }

    pub fn f32_suffixed(value: f32) -> Self {
        Self::External(proc_macro2::Literal::f32_suffixed(value))
    }

    pub fn f64_suffixed(value: f64) -> Self {
        Self::External(proc_macro2::Literal::f64_suffixed(value))
    }

    pub fn u8_unsuffixed(value: u8) -> Self {
        Self::External(proc_macro2::Literal::u8_unsuffixed(value))
    }

    pub fn u16_unsuffixed(value: u16) -> Self {
        Self::External(proc_macro2::Literal::u16_unsuffixed(value))
    }

    pub fn u32_unsuffixed(value: u32) -> Self {
        Self::External(proc_macro2::Literal::u32_unsuffixed(value))
    }

    pub fn u64_unsuffixed(value: u64) -> Self {
        Self::External(proc_macro2::Literal::u64_unsuffixed(value))
    }

    pub fn usize_unsuffixed(value: usize) -> Self {
        Self::External(proc_macro2::Literal::usize_unsuffixed(value))
    }

    pub fn i8_unsuffixed(value: i8) -> Self {
        Self::External(proc_macro2::Literal::i8_unsuffixed(value))
    }

    pub fn i16_unsuffixed(value: i16) -> Self {
        Self::External(proc_macro2::Literal::i16_unsuffixed(value))
    }

    pub fn i32_unsuffixed(value: i32) -> Self {
        Self::External(proc_macro2::Literal::i32_unsuffixed(value))
    }

    pub fn i64_unsuffixed(value: i64) -> Self {
        Self::External(proc_macro2::Literal::i64_unsuffixed(value))
    }

    pub fn isize_unsuffixed(value: isize) -> Self {
        Self::External(proc_macro2::Literal::isize_unsuffixed(value))
    }

    pub fn f32_unsuffixed(value: f32) -> Self {
        Self::External(proc_macro2::Literal::f32_unsuffixed(value))
    }

    pub fn f64_unsuffixed(value: f64) -> Self {
        Self::External(proc_macro2::Literal::f64_unsuffixed(value))
    }

    pub fn span(&self) -> Span {
        match self {
            Self::External(v) => v.span().into(),
            Self::Internal { span, .. } => *span,
        }
    }

    pub fn set_span(&mut self, span: Span) {
        match self {
            Self::External(v) => v.set_span(span.into()),
            Self::Internal { span: s, .. } => *s = span,
        }
    }
}

impl From<proc_macro2::Literal> for Literal {
    fn from(value: proc_macro2::Literal) -> Self {
        Self::External(value)
    }
}

impl From<Literal> for proc_macro2::Literal {
    fn from(value: Literal) -> Self {
        match value {
            Literal::External(v) => v,
            Literal::Internal { repr, span } => {
                let mut lit: proc_macro2::Literal = repr
                    .parse()
                    .unwrap_or_else(|_| proc_macro2::Literal::string(&repr));
                lit.set_span(span.into());
                lit
            }
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::External(v) => write!(f, "{}", v),
            Self::Internal { repr, .. } => write!(f, "{}", repr),
        }
    }
}
