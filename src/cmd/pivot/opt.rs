use structopt::StructOpt;
use std::path::PathBuf;

// const DEFAULT_PIVOT_CONFIG_PATH: &str = ".config/pivot/config.yaml";

#[derive(Debug, StructOpt)]
#[structopt(name = "pivot")]
pub struct PivotOpt {

    /// Target profile to load
    #[structopt(short, long)]
    target: String,

    /// Override path to configuration file
    #[structopt(short, long, parse(from_os_str))]
    cfg_path: Option<PathBuf>,
}