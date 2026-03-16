use std::path::PathBuf;

#[derive(clap::Parser, Debug, Default, Clone)]
#[command(version, about, long_about)]
pub struct Args {
    #[arg(default_value = "schema.yml", value_name = "PATH", value_hint = clap::ValueHint::DirPath)]
    pub path: PathBuf,

    #[arg(long, short, default_value = "output", value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    pub output: PathBuf,

    #[arg(long, short, value_enum, value_delimiter = ' ', num_args = 0..)]
    pub features: Vec<Feature>,

    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

#[derive(clap::ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Feature {
    Visit,
    Expand,
}
