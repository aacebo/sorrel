use super::{Base, Field};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    #[serde(rename = "$key$")]
    pub name: String,

    #[serde(default)]
    pub extends: Base,

    #[serde(default)]
    pub fields: Vec<Field>,

    #[serde(default)]
    pub doc: Option<String>,
}
