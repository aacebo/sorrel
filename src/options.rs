use std::path::PathBuf;

#[derive(clap::Parser, Debug, Default, Clone)]
#[command(version, about, long_about = None)]
pub struct Options {
    #[arg(long, short, default_value = "schema.yml")]
    pub path: PathBuf,

    #[arg(long, short, default_value = "output")]
    pub output: PathBuf,

    #[arg(long, short, value_enum, value_delimiter = ' ', num_args = 0..)]
    pub features: Vec<Feature>,
}

#[derive(clap::ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Feature {
    Visit,
    Expand,
}
