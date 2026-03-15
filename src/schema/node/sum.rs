use super::{Field, Variant};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SumVariant {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sum {
    #[serde(rename = "$key$")]
    pub name: String,

    #[serde(default)]
    pub variants: Vec<Variant>,

    #[serde(default)]
    pub doc: Option<String>,
}
