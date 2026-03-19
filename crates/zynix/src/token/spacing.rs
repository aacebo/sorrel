#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Spacing {
    #[default]
    Alone,
    Joint,
}

impl Spacing {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Alone => "alone",
            Self::Joint => "joint",
        }
    }
}

impl From<proc_macro2::Spacing> for Spacing {
    fn from(value: proc_macro2::Spacing) -> Self {
        match value {
            proc_macro2::Spacing::Alone => Self::Alone,
            proc_macro2::Spacing::Joint => Self::Joint,
        }
    }
}

impl From<Spacing> for proc_macro2::Spacing {
    fn from(value: Spacing) -> Self {
        match value {
            Spacing::Alone => proc_macro2::Spacing::Alone,
            Spacing::Joint => proc_macro2::Spacing::Joint,
        }
    }
}

impl From<proc_macro::Spacing> for Spacing {
    fn from(value: proc_macro::Spacing) -> Self {
        match value {
            proc_macro::Spacing::Alone => Self::Alone,
            proc_macro::Spacing::Joint => Self::Joint,
        }
    }
}

impl From<Spacing> for proc_macro::Spacing {
    fn from(value: Spacing) -> Self {
        match value {
            Spacing::Alone => proc_macro::Spacing::Alone,
            Spacing::Joint => proc_macro::Spacing::Joint,
        }
    }
}
