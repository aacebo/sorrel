#![feature(iterator_try_collect)]

mod args;
mod schema;
mod source;

use clap::Parser;
use std::fs;

pub use args::*;
pub use schema::*;
pub use source::*;

pub trait ToClapError {
    fn to_clap_error(self) -> clap::Error;
}

impl ToClapError for syn::Error {
    fn to_clap_error(self) -> clap::Error {
        clap::Error::raw(clap::error::ErrorKind::InvalidValue, self.to_string())
    }
}

pub trait ClapErrorExt {
    fn with_context(self, kind: clap::error::ContextKind, value: clap::error::ContextValue)
    -> Self;
}

impl ClapErrorExt for clap::Error {
    fn with_context(
        mut self,
        kind: clap::error::ContextKind,
        value: clap::error::ContextValue,
    ) -> Self {
        self.insert(kind, value);
        self
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = fs::read_to_string(&args.path)?;
    let schema: Schema = serde_yml::from_str(&file)?;
    let sources = match schema.run(&args) {
        Err(err) => {
            for (_, value) in err.context() {
                println!("{}", value);
            }

            err.exit()
        }
        Ok(v) => v,
    };

    sources.save()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    use std::fs;

    #[test]
    fn parse_schema() {
        let file = fs::read_to_string("schema.yml").unwrap();
        let schema: Schema = serde_yml::from_str(&file).unwrap();
        println!("{:#?}", &schema);
    }
}
