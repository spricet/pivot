use std::fs::File;
use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;
use validator::Validate;

use pivot::config::v1alpha::Config;
use pivot::switch::Switcher;

use crate::opt::PivotOpt;

mod opt;

const DEFAULT_PIVOT_CONFIG_PATH: &str = ".config/pivot/config.yaml";

fn main() {
    let opts: PivotOpt = PivotOpt::from_args();
    let cfg_path = match opts.cfg_path {
        Some(p) => p,
        None => PathBuf::from(std::env::var("HOME").expect("missing HOME env"))
            .join(DEFAULT_PIVOT_CONFIG_PATH),
    };

    let p = cfg_path.clone();
    let p = p.to_string_lossy();
    let file =
        File::open(cfg_path).unwrap_or_else(|_| panic!("unable to open pivot config: {}", p));
    let cfg: Config = serde_yaml::from_reader(file)
        .unwrap_or_else(|e| panic!("failed to parse pivot config: {} - {}", p, e));
    if let Err(e) = cfg.validate() {
        println!("invalid pivot config: {} - {}", p, e);
        exit(1);
    }

    let switcher = Switcher::new(cfg);
    match switcher.switch(&opts.target) {
        Ok(_) => {}
        Err(e) => println!("command failed: {:?}", e),
    }
}
