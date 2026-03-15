use std::path::PathBuf;

#[derive(clap::Parser, Debug, Default, Clone)]
#[command(version, about, long_about = None)]
pub struct Options {
    #[arg(long, short, default_value = "schema.yml")]
    pub path: PathBuf,

    #[arg(long, short)]
    pub features: Vec<String>,
}
