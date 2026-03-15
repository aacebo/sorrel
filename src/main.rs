pub mod options;
pub mod schema;

use std::fs;
use clap::Parser;

pub use options::*;
pub use schema::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::parse();
    let file = fs::read_to_string(&options.path)?;
    let _: Schema = serde_yml::from_str(&file)?;
    println!("{:#?}", &options);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    use std::fs;

    #[test]
    fn should_parse_schema() {
        let file = fs::read_to_string("schema.yml").unwrap();
        let schema: Schema = serde_yml::from_str(&file).unwrap();
        println!("{:#?}", &schema);
    }
}
