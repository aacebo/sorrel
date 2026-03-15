mod opaque;
mod product;
mod sum;

pub use opaque::*;
pub use product::*;
pub use sum::*;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub enum Base {
    #[default]
    #[serde(rename = "node_base")]
    Node,

    #[serde(rename = "attributed_node_base")]
    Attributed,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Variant {
    Enum(String),
    Sum(SumVariant),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Node {
    Opaque(Opaque),
    Product(Product),
    Sum(Sum),
}

impl Node {
    #[allow(unused)]
    pub fn name(&self) -> &str {
        match self {
            Self::Opaque(v) => &v.name,
            Self::Product(v) => &v.name,
            Self::Sum(v) => &v.name,
        }
    }

    #[allow(unused)]
    pub fn doc(&self) -> Option<&str> {
        match self {
            Self::Opaque(_) => None,
            Self::Product(v) => v.doc.as_deref(),
            Self::Sum(v) => v.doc.as_deref(),
        }
    }
}
