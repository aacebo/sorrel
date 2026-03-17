use crate::Span;

#[derive(Debug, Clone)]
pub struct Literal {
    repr: String,
    span: Span,
}

impl Literal {
    pub fn string(value: &str) -> Self {
        let pm = proc_macro2::Literal::string(value);
        Self {
            repr: pm.to_string(),
            span: Span::call_site(),
        }
    }

    pub fn character(value: char) -> Self {
        let pm = proc_macro2::Literal::character(value);
        Self {
            repr: pm.to_string(),
            span: Span::call_site(),
        }
    }

    pub fn u8_suffixed(value: u8) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::u8_suffixed(value))
    }

    pub fn u16_suffixed(value: u16) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::u16_suffixed(value))
    }

    pub fn u32_suffixed(value: u32) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::u32_suffixed(value))
    }

    pub fn u64_suffixed(value: u64) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::u64_suffixed(value))
    }

    pub fn usize_suffixed(value: usize) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::usize_suffixed(value))
    }

    pub fn i8_suffixed(value: i8) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::i8_suffixed(value))
    }

    pub fn i16_suffixed(value: i16) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::i16_suffixed(value))
    }

    pub fn i32_suffixed(value: i32) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::i32_suffixed(value))
    }

    pub fn i64_suffixed(value: i64) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::i64_suffixed(value))
    }

    pub fn isize_suffixed(value: isize) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::isize_suffixed(value))
    }

    pub fn f32_suffixed(value: f32) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::f32_suffixed(value))
    }

    pub fn f64_suffixed(value: f64) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::f64_suffixed(value))
    }

    pub fn u8_unsuffixed(value: u8) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::u8_unsuffixed(value))
    }

    pub fn u16_unsuffixed(value: u16) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::u16_unsuffixed(value))
    }

    pub fn u32_unsuffixed(value: u32) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::u32_unsuffixed(value))
    }

    pub fn u64_unsuffixed(value: u64) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::u64_unsuffixed(value))
    }

    pub fn usize_unsuffixed(value: usize) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::usize_unsuffixed(value))
    }

    pub fn i8_unsuffixed(value: i8) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::i8_unsuffixed(value))
    }

    pub fn i16_unsuffixed(value: i16) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::i16_unsuffixed(value))
    }

    pub fn i32_unsuffixed(value: i32) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::i32_unsuffixed(value))
    }

    pub fn i64_unsuffixed(value: i64) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::i64_unsuffixed(value))
    }

    pub fn isize_unsuffixed(value: isize) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::isize_unsuffixed(value))
    }

    pub fn f32_unsuffixed(value: f32) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::f32_unsuffixed(value))
    }

    pub fn f64_unsuffixed(value: f64) -> Self {
        Self::from_proc_macro(proc_macro2::Literal::f64_unsuffixed(value))
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    fn from_proc_macro(lit: proc_macro2::Literal) -> Self {
        Self {
            repr: lit.to_string(),
            span: Span::call_site(),
        }
    }
}

impl From<proc_macro2::Literal> for Literal {
    fn from(value: proc_macro2::Literal) -> Self {
        Self {
            repr: value.to_string(),
            span: value.span().into(),
        }
    }
}

impl From<Literal> for proc_macro2::Literal {
    fn from(value: Literal) -> Self {
        // Parse the repr back into a proc_macro2::Literal
        let mut lit: proc_macro2::Literal = value
            .repr
            .parse()
            .unwrap_or_else(|_| proc_macro2::Literal::string(&value.repr));
        lit.set_span(value.span.into());
        lit
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.repr)
    }
}
