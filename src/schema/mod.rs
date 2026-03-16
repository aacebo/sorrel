use serde_with::{KeyValueMap, serde_as};

use crate::Args;

mod node;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Schema {
    #[serde(default = "Schema::default_name")]
    pub name: String,

    #[serde(default = "Schema::default_version")]
    pub version: semver::Version,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub nodes: Nodes,
}

impl Schema {
    fn default_name() -> String {
        "rust_ast".to_string()
    }

    fn default_version() -> semver::Version {
        "0.0.0".parse().unwrap()
    }
}

impl Schema {
    pub fn run(&self, args: &Args) -> Result<proc_macro2::TokenStream, clap::Error> {
        let mut tokens = proc_macro2::TokenStream::new();

        for node in self.nodes.iter() {
            tokens.extend(node.run(args)?);
        }

        Ok(tokens)
    }
}

#[serde_as]
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Nodes {
    #[serde(flatten)]
    #[serde_as(as = "KeyValueMap<_>")]
    items: Vec<node::Node>,
}

impl std::ops::Deref for Nodes {
    type Target = [node::Node];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
