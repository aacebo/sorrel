#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Opaque {
    #[serde(rename = "$key$")]
    pub name: String,
}
