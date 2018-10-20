extern crate hltb;
extern crate structopt;

use hltb::{run, Error, Opt};
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let res = run(&opt);
    // TODO pretty print results (as table)
    println!("{:?}", res);
    Ok(())
}
