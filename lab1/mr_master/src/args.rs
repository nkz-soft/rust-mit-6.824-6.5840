use clap::Parser;
use mr_common::Configuration;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the files
    #[arg(default_value = ".", short = 'f', long = "file-path")]
    pub path_to_files: PathBuf,

    /// Number of reduce tasks
    #[arg(default_value = "10", short = 'r', long = "reduce-task-num")]
    pub reduce_task_num: u32,
}

impl From<Args> for Configuration {
    fn from(val: Args) -> Self {
        Configuration::new(val.path_to_files, val.reduce_task_num)
    }
}
