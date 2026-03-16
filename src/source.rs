use std::{collections::BTreeMap, path::PathBuf};

use quote::{format_ident, quote};

#[derive(Debug, Clone)]
pub struct SourceMap {
    sources: BTreeMap<String, Source>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            sources: BTreeMap::default(),
        }
    }

    pub fn len(&self) -> usize {
        self.sources.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    pub fn has(&self, ident: &str) -> bool {
        self.sources.contains_key(ident)
    }

    pub fn get(&self, ident: &str) -> Option<&Source> {
        self.sources.get(ident)
    }

    pub fn get_mut(&mut self, ident: &str) -> Option<&mut Source> {
        self.sources.get_mut(ident)
    }

    pub fn set(&mut self, ident: &str, value: impl Into<Source>) {
        self.sources.insert(ident.to_string(), value.into());
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Source)> {
        self.sources.iter()
    }

    pub fn save(&self, path: &PathBuf) -> std::io::Result<()> {
        let mut root = vec![];

        for src in self.sources.values() {
            let ident = format_ident!("{}", &src.module);
            src.save()?;
            root.push(quote!(
                mod #ident;
                pub use #ident::*;
            ));
        }

        std::fs::write(
            path.join("lib.rs"),
            quote! {
                pub use zyn_token::*;
                #(#root)*
            }
            .to_string(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Source {
    pub file: PathBuf,
    pub module: String,
    pub content: proc_macro2::TokenStream,
}

impl Source {
    pub fn dir(&self) -> PathBuf {
        self.file
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or(PathBuf::default())
    }

    pub fn save(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.dir())?;
        std::fs::write(&self.file, self.content.to_string())?;
        Ok(())
    }
}
