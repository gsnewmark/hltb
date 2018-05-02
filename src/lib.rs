#[macro_use]
extern crate structopt;

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Opt {
    #[structopt(name = "GAME")]
    game: String,
}
