#![feature(iterator_try_collect)]

mod args;
mod schema;

use clap::Parser;
use std::fs;

pub use args::*;
pub use schema::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = fs::read_to_string(&args.path)?;
    let schema: Schema = serde_yml::from_str(&file)?;
    let tokens = schema.run(&args)?;
    println!("{:#?}", &tokens);
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
