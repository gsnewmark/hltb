extern crate hltb;
extern crate structopt;

use hltb::{run, Opt};
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    let res = run(&opt);
    println!("{:?}", res);
}
