use crate::opt::PivotOpt;
use structopt::StructOpt;

mod opt;

fn main() {
    let options = PivotOpt::from_args();
    println!("{:#?}", options)
}