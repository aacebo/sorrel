use serde_with::{KeyValueMap, serde_as};

mod node;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Schema {
    pub name: String,
    pub version: String,
    pub description: String,

    #[serde(default)]
    pub nodes: Nodes,
}

#[serde_as]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Nodes {
    #[serde(flatten)]
    #[serde_as(as = "KeyValueMap<_>")]
    items: Vec<node::Node>,
}
