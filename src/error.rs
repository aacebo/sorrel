use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Error {
    inner: AnyError,
    items: BTreeMap<String, String>,
}

impl Error {
    pub fn inner(&self) -> &AnyError {
        &self.inner
    }

    pub fn has(&self, key: &str) -> bool {
        self.items.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.items.get(key).map(|v| v.as_str())
    }

    pub fn context(&self) -> impl Iterator<Item = (&String, &String)> {
        self.items.iter()
    }

    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.items.insert(key.into(), value.into());
        self
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.items.insert(key.into(), value.into());
        self
    }

    pub fn exit(&self) -> ! {
        eprintln!("{}", self);
        std::process::exit(1)
    }
}

impl From<AnyError> for Error {
    fn from(inner: AnyError) -> Self {
        Self {
            inner,
            items: BTreeMap::default(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        AnyError::from(value).into()
    }
}

impl From<clap::Error> for Error {
    fn from(value: clap::Error) -> Self {
        AnyError::from(value).into()
    }
}

impl From<syn::Error> for Error {
    fn from(value: syn::Error) -> Self {
        AnyError::from(value).into()
    }
}

impl From<proc_macro2::LexError> for Error {
    fn from(value: proc_macro2::LexError) -> Self {
        AnyError::from(value).into()
    }
}

impl From<serde_yml::Error> for Error {
    fn from(value: serde_yml::Error) -> Self {
        AnyError::from(value).into()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.items {
            writeln!(f, "{} => {}", key, value)?;
        }

        if !self.items.is_empty() {
            writeln!(f)?;
        }

        write!(f, "Error: {}", &self.inner)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

#[derive(Debug)]
pub enum AnyError {
    IO(std::io::Error),
    Clap(clap::Error),
    Syn(syn::Error),
    Lex(proc_macro2::LexError),
    Yml(serde_yml::Error),
}

impl AnyError {
    pub fn as_io(&self) -> Option<&std::io::Error> {
        match self {
            Self::IO(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_clap(&self) -> Option<&clap::Error> {
        match self {
            Self::Clap(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_syn(&self) -> Option<&syn::Error> {
        match self {
            Self::Syn(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_lex(&self) -> Option<&proc_macro2::LexError> {
        match self {
            Self::Lex(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_yml(&self) -> Option<&serde_yml::Error> {
        match self {
            Self::Yml(v) => Some(v),
            _ => None,
        }
    }
}

impl From<std::io::Error> for AnyError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<clap::Error> for AnyError {
    fn from(value: clap::Error) -> Self {
        Self::Clap(value)
    }
}

impl From<syn::Error> for AnyError {
    fn from(value: syn::Error) -> Self {
        Self::Syn(value)
    }
}

impl From<proc_macro2::LexError> for AnyError {
    fn from(value: proc_macro2::LexError) -> Self {
        Self::Lex(value)
    }
}

impl From<serde_yml::Error> for AnyError {
    fn from(value: serde_yml::Error) -> Self {
        Self::Yml(value)
    }
}

impl std::fmt::Display for AnyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(v) => write!(f, "{}", v),
            Self::Clap(v) => write!(f, "{}", v.render().ansi()),
            Self::Syn(v) => write!(f, "{}", v),
            Self::Lex(v) => write!(f, "{}", v),
            Self::Yml(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for AnyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IO(v) => Some(v),
            Self::Clap(v) => Some(v),
            Self::Syn(v) => Some(v),
            Self::Lex(v) => Some(v),
            Self::Yml(v) => Some(v),
        }
    }
}

pub trait ToError {
    fn to_error(self) -> Error;
}

impl ToError for std::io::Error {
    fn to_error(self) -> Error {
        self.into()
    }
}

impl ToError for clap::Error {
    fn to_error(self) -> Error {
        self.into()
    }
}

impl ToError for syn::Error {
    fn to_error(self) -> Error {
        self.into()
    }
}

impl ToError for proc_macro2::LexError {
    fn to_error(self) -> Error {
        self.into()
    }
}

impl ToError for serde_yml::Error {
    fn to_error(self) -> Error {
        self.into()
    }
}

impl<T> From<Error> for Result<T, Error> {
    fn from(value: Error) -> Self {
        Self::Err(value)
    }
}
