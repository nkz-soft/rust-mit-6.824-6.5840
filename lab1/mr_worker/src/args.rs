use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the plugin
    #[arg(short = 'p', long = "plugin")]
    pub plugin: PathBuf,
}
