mod product;
mod sum;

pub use product::*;
use quote::{format_ident, quote};
pub use sum::*;

use crate::{Args, ClapErrorExt, ToClapError};

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

    pub fn run(&self, _args: &Args) -> Result<proc_macro2::TokenStream, clap::Error> {
        let ident = format_ident!("{}", &self.name);
        let kind: syn::Path = match syn::parse_str(&self.kind) {
            Err(err) => {
                return Err(err.to_clap_error().with_context(
                    clap::error::ContextKind::Custom,
                    clap::error::ContextValue::Strings(vec![
                        "field".to_string(),
                        self.name.clone(),
                        self.kind.clone(),
                    ]),
                ));
            }
            Ok(v) => v,
        };

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
    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, clap::Error> {
        Ok(match self {
            Self::Enum(name) => {
                let ident = format_ident!("{}", &name);
                quote!(#ident)
            }
            Self::Sum(v) => v.run(args)?,
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
    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, clap::Error> {
        match self {
            Self::Product(v) => v.run(args),
            Self::Sum(v) => v.run(args),
        }
    }
}
