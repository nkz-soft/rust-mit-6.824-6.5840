use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub(crate) struct Args {
    /// Path to the plugin
    #[arg(default_value = "mr_wc.dll", short = 'p', long = "plugin")]
    pub(crate) plugin: PathBuf,
}
