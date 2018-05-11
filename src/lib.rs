#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

extern crate reqwest;

#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Opt {
    #[structopt(name = "GAME")]
    game: String,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unknown error")]
    Unknown,
}

pub fn run(opt: &Opt) -> Result<reqwest::Response, Error> {
    reqwest::get("https://howlongtobeat.com").map_err(|_| Error::Unknown)
}
