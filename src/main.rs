#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

use std::num::ParseIntError;

use failure::Error;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(name = "GAME")]
    game: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
