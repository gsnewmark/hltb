#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Opt {
    #[structopt(name = "GAME")]
    game: String,
}
