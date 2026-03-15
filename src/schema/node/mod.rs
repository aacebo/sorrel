mod product;
mod sum;

pub use product::*;
use quote::quote;
pub use sum::*;

use crate::Options;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub enum Base {
    #[default]
    #[serde(rename = "node_base")]
    Node,

    #[serde(rename = "attributed_node_base")]
    Attributed,
}

impl Base {
    pub fn fields(&self) -> Vec<Field> {
        match self {
            Self::Node => vec![Field::span()],
            Self::Attributed => vec![Field::span(), Field::attrs()],
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Field {
    pub name: String,

    #[serde(rename = "type")]
    pub kind: String,

    #[serde(default)]
    pub doc: Option<String>,

    #[serde(default)]
    pub default: Option<serde_yml::Value>,
}

impl Field {
    pub fn span() -> Self {
        Self {
            name: "span".to_string(),
            kind: "Span".to_string(),
            doc: None,
            default: None,
        }
    }

    pub fn attrs() -> Self {
        Self {
            name: "attrs".to_string(),
            kind: "Vec<Attribute>".to_string(),
            doc: None,
            default: None,
        }
    }

    pub fn run(&self, _options: &Options) -> Result<proc_macro2::TokenStream, clap::Error> {
        let ident = &self.name;
        let kind = &self.kind;

        Ok(quote! {
            pub #ident: #kind,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Variant {
    Enum(String),
    Sum(SumVariant),
}

impl Variant {
    pub fn run(&self, options: &Options) -> Result<proc_macro2::TokenStream, clap::Error> {
        Ok(match self {
            Self::Enum(ident) => quote!(#ident),
            Self::Sum(v) => v.run(options)?,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Node {
    Product(Product),
    Sum(Sum),
}

impl Node {
    #[allow(unused)]
    pub fn name(&self) -> &str {
        match self {
            Self::Product(v) => &v.name,
            Self::Sum(v) => &v.name,
        }
    }

    #[allow(unused)]
    pub fn doc(&self) -> Option<&str> {
        match self {
            Self::Product(v) => v.doc.as_deref(),
            Self::Sum(v) => v.doc.as_deref(),
        }
    }

    #[allow(unused)]
    pub fn run(&self, options: &Options) -> Result<proc_macro2::TokenStream, clap::Error> {
        match self {
            Self::Product(v) => v.run(options),
            _ => Err(clap::Error::raw(
                clap::error::ErrorKind::UnknownArgument,
                "unknown argument",
            )),
        }
    }
}
