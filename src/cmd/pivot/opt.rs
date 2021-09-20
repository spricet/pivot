use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pivot")]
pub struct PivotOpt {
    /// Target profile to load
    #[structopt(short, long)]
    pub target: String,

    /// Override path to configuration file
    #[structopt(short, long, parse(from_os_str))]
    pub cfg_path: Option<PathBuf>,
}
