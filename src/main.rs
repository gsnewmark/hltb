extern crate hltb;
extern crate structopt;

use hltb::Opt;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
