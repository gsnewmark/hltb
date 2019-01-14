#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate prettytable;

use hltb as lib;
use prettytable::Table;
use structopt::StructOpt;

fn main() -> Result<(), lib::Error> {
    setup_panic!();
    let opt = lib::Opt::from_args();
    let res = lib::run(&opt)?;

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
