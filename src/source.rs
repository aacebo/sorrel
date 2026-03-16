use std::{collections::BTreeMap, path::PathBuf};

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

    pub fn save(&self) -> std::io::Result<()> {
        for src in self.sources.values() {
            src.save()?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Source {
    pub path: PathBuf,
    pub content: proc_macro2::TokenStream,
}

impl Source {
    pub fn dir(&self) -> PathBuf {
        self.path
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or(PathBuf::default())
    }

    pub fn save(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.dir())?;
        std::fs::write(&self.path, self.content.to_string())?;
        Ok(())
    }
}
