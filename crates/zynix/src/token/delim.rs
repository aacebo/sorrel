#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Delim {
    #[default]
    None,
    Paren,
    Brace,
    Bracket,
}

impl Delim {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Paren => "paren",
            Self::Brace => "brace",
            Self::Bracket => "bracket",
            Self::None => "none",
        }
    }
}

impl From<proc_macro::Delimiter> for Delim {
    fn from(value: proc_macro::Delimiter) -> Self {
        match value {
            proc_macro::Delimiter::Parenthesis => Self::Paren,
            proc_macro::Delimiter::Brace => Self::Brace,
            proc_macro::Delimiter::Bracket => Self::Bracket,
            proc_macro::Delimiter::None => Self::None,
        }
    }
}

impl From<Delim> for proc_macro::Delimiter {
    fn from(value: Delim) -> Self {
        match value {
            Delim::Paren => proc_macro::Delimiter::Parenthesis,
            Delim::Brace => proc_macro::Delimiter::Brace,
            Delim::Bracket => proc_macro::Delimiter::Bracket,
            Delim::None => proc_macro::Delimiter::None,
        }
    }
}
