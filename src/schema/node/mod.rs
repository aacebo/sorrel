mod product;
mod sum;

pub use product::*;
pub use sum::*;

use quote::{format_ident, quote};

use crate::{Args, Error, ToError};

// ── Meta types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum Meta {
    Type(TypeMeta),
    Field(FieldMeta),
}

/// Meta attached to a node (product/sum) definition.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct TypeMeta {
    #[serde(default)]
    pub public_leaf: bool,
    #[serde(default)]
    pub macro_facing: bool,
    #[serde(default)]
    pub notes: Option<String>,
}

/// Meta attached to a field definition.
#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct FieldMeta {
    /// Whether this field holds an AST node that should be traversed by Visit/Fold.
    #[serde(default)]
    pub node: bool,
    /// How the field value is wrapped — determines the emitted Rust type and traversal pattern.
    #[serde(default)]
    pub wrapper: Wrapper,
}

/// Describes how a node-typed field is wrapped in the generated Rust struct.
/// The generator uses this to both emit the correct type and generate traversal code.
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Wrapper {
    /// `T` — plain value
    #[default]
    None,
    /// `Box<T>`
    Box,
    /// `Vec<T>`
    Vec,
    /// `Option<T>`
    Option,
    /// `Option<Box<T>>`
    OptionBox,
    /// `Option<Vec<T>>`
    OptionVec,
    /// `crate::ast::Punctuated<T, P>` — separator type P is provided separately in the `type` field
    Punctuated,
}

impl FieldMeta {
    /// Produces the Rust type string for a field given its bare inner type and wrapper.
    pub fn rust_type<'a>(&self, inner: &'a str) -> std::borrow::Cow<'a, str> {
        match self.wrapper {
            Wrapper::None => std::borrow::Cow::Borrowed(inner),
            Wrapper::Box => std::borrow::Cow::Owned(format!("Box<{}>", inner)),
            Wrapper::Vec => std::borrow::Cow::Owned(format!("Vec<{}>", inner)),
            Wrapper::Option => std::borrow::Cow::Owned(format!("Option<{}>", inner)),
            Wrapper::OptionBox => std::borrow::Cow::Owned(format!("Option<Box<{}>>", inner)),
            Wrapper::OptionVec => std::borrow::Cow::Owned(format!("Option<Vec<{}>>", inner)),
            Wrapper::Punctuated => std::borrow::Cow::Borrowed(inner), // full type already in schema
        }
    }
}

// ── Base ──────────────────────────────────────────────────────────────────────

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

// ── Field ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Field {
    pub name: String,

    #[serde(rename = "type")]
    pub kind: String,

    #[serde(default)]
    pub doc: Option<String>,

    #[serde(default)]
    pub default: Option<serde_yml::Value>,

    #[serde(default)]
    pub meta: Option<FieldMeta>,
}

impl Field {
    pub fn span() -> Self {
        Self {
            name: "span".to_string(),
            kind: "crate::Span".to_string(),
            doc: None,
            default: None,
            meta: None,
        }
    }

    pub fn attrs() -> Self {
        Self {
            name: "attrs".to_string(),
            kind: "Vec<Attribute>".to_string(),
            doc: None,
            default: None,
            meta: None,
        }
    }

    /// Returns the Rust type string for this field, applying any wrapper declared in meta.
    pub fn rust_type(&self) -> std::borrow::Cow<'_, str> {
        match &self.meta {
            Some(m) if m.node => m.rust_type(&self.kind),
            _ => std::borrow::Cow::Borrowed(&self.kind),
        }
    }

    pub fn run(&self, _args: &Args) -> Result<proc_macro2::TokenStream, Error> {
        let ident = format_ident!("{}", &self.name);
        let type_str = self.rust_type();
        let kind: syn::Type = match syn::parse_str(type_str.as_ref()) {
            Err(err) => {
                return err
                    .to_error()
                    .with("entity", "field")
                    .with("name", &self.name)
                    .with("type", type_str.as_ref())
                    .into();
            }
            Ok(v) => v,
        };

        let doc = self.doc.as_deref().map(|d| quote!(#[doc = #d]));

        Ok(quote! {
            #doc
            #ident: #kind
        })
    }
}

// ── Variant ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum Variant {
    Enum(String),
    Sum(SumVariant),
}

impl Variant {
    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, Error> {
        Ok(match self {
            Self::Enum(name) => {
                let ident = format_ident!("{}", &name);
                quote!(#ident)
            }
            Self::Sum(v) => v.run(args)?,
        })
    }
}

// ── Node ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Node {
    Product(Product),
    Sum(Sum),
}

impl Node {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Product(_) => "product",
            Self::Sum(_) => "sum",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Product(v) => &v.name,
            Self::Sum(v) => &v.name,
        }
    }

    pub fn doc(&self) -> Option<&str> {
        match self {
            Self::Product(v) => v.doc.as_deref(),
            Self::Sum(v) => v.doc.as_deref(),
        }
    }

    pub fn submodule(&self) -> Option<&str> {
        match self {
            Self::Product(v) => v.submodule.as_deref(),
            Self::Sum(v) => v.submodule.as_deref(),
        }
    }

    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, Error> {
        match self {
            Self::Product(v) => v.run(args),
            Self::Sum(v) => v.run(args),
        }
    }
}
