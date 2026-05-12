use check_keyword::CheckKeyword;
use serde_with::{KeyValueMap, serde_as};

use crate::{Args, Error, Source, SourceMap};

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
    pub fn run(&self, args: &Args) -> Result<SourceMap, Error> {
        let mut map = SourceMap::new();

        for node in self.nodes.iter() {
            let mut module = convert_case::ccase!(snake, node.name());

            if module.is_keyword() {
                module = format!("_{}", &module);
            }

            let submodule = node.submodule().map(|s| s.to_string());

            let file = match &submodule {
                Some(sub) if node.submodule().is_some() && matches!(node, node::Node::Sum(_)) => {
                    args.output.join(sub).join("mod.rs")
                }
                Some(sub) => args.output.join(sub).join(format!("{}.rs", &module)),
                None => args.output.join(format!("{}.rs", &module)),
            };

            map.set(
                node.name(),
                Source {
                    file,
                    module,
                    submodule,
                    content: node.run(args)?,
                },
            );
        }

        Ok(map)
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
