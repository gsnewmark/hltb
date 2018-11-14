#[macro_use]
extern crate prettytable;
extern crate hltb;
extern crate structopt;

use hltb::{run, Error, Opt};
use prettytable::Table;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let res = run(&opt)?;

    let mut table = Table::new();
    table.add_row(row![
        "Title",
        "Main Time",
        "Main + Extra Time",
        "Completionist Time",
        "URL"
    ]);
    for game in &res {
        table.add_row(row![
            game.title,
            game.main_story_time,
            game.main_extra_time,
            game.completionist_time,
            game.hltb_url
        ]);
    }
    table.printstd();

    Ok(())
}
