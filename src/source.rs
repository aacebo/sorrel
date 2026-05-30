use std::collections::BTreeMap;

use quote::{format_ident, quote};

#[derive(Debug, Default, Clone)]
pub struct SourceMap {
    sources: BTreeMap<String, Source>,
}

impl SourceMap {
    pub fn new() -> Self {
        Self::default()
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

    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        // Group sources by their submodule (None = top-level).
        let mut by_submodule: BTreeMap<Option<String>, Vec<&Source>> = BTreeMap::new();

        for src in self.sources.values() {
            by_submodule
                .entry(src.submodule.clone())
                .or_default()
                .push(src);
        }

        let mut root_decls = vec![];

        for (submodule, sources) in &by_submodule {
            match submodule {
                None => {
                    // Top-level files — write each and declare in root mod.rs.
                    for src in sources {
                        let ident = format_ident!("{}", &src.module);
                        src.save()?;
                        root_decls.push(quote!(
                            mod #ident;
                            pub use #ident::*;
                        ));
                    }
                }
                Some(sub) => {
                    // Submodule — write each file, then write the subdir mod.rs.
                    let sub_dir = path.join(sub);
                    std::fs::create_dir_all(&sub_dir)?;

                    let mut sub_decls = vec![];
                    for src in sources {
                        src.save()?;
                        // The submodule root (mod.rs) is declared via the subdir, not as a file.
                        if src.file.file_name() == Some(std::ffi::OsStr::new("mod.rs")) {
                            // Content already written; its declarations come from sub_decls below.
                            continue;
                        }
                        let ident = format_ident!("{}", &src.module);
                        sub_decls.push(quote!(
                            mod #ident;
                            pub use #ident::*;
                        ));
                    }

                    // Write the subdir mod.rs.
                    // Find if there's an explicit mod.rs source (the sum enum itself).
                    let has_root = sources
                        .iter()
                        .any(|s| s.file.file_name() == Some(std::ffi::OsStr::new("mod.rs")));

                    if has_root {
                        // The sum enum source already has its content — append child re-exports.
                        let root_src = sources
                            .iter()
                            .find(|s| s.file.file_name() == Some(std::ffi::OsStr::new("mod.rs")))
                            .unwrap();
                        let existing = &root_src.content;
                        std::fs::write(
                            &root_src.file,
                            quote!(#existing #(#sub_decls)*).to_string(),
                        )?;
                    } else {
                        std::fs::write(sub_dir.join("mod.rs"), quote!(#(#sub_decls)*).to_string())?;
                    }

                    let sub_ident = format_ident!("{}", sub);
                    root_decls.push(quote!(
                        mod #sub_ident;
                        pub use #sub_ident::*;
                    ));
                }
            }
        }

        std::fs::write(
            path.join("mod.rs"),
            quote!(#![allow(unused)] pub use crate::ast::leaf::*; #(#root_decls)*).to_string(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Source {
    pub file: std::path::PathBuf,
    pub module: String,
    pub submodule: Option<String>,
    pub content: proc_macro2::TokenStream,
}

impl Source {
    pub fn dir(&self) -> std::path::PathBuf {
        self.file
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_default()
    }

    pub fn save(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(self.dir())?;
        std::fs::write(&self.file, self.content.to_string())?;
        Ok(())
    }
}
