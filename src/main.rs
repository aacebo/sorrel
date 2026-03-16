#![feature(iterator_try_collect)]

mod args;
mod error;
mod schema;
mod source;

use clap::Parser;
use std::fs;

pub use args::*;
pub use error::*;
pub use schema::*;
pub use source::*;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let file = fs::read_to_string(&args.path)?;
    let schema: Schema = serde_yml::from_str(&file)?;
    let sources = match schema.run(&args) {
        Err(err) => err.exit(),
        Ok(v) => v,
    };

    if !args.dry_run {
        sources.save(&args.output)?;
    }

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
