#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};

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
    #[fail(display = "HLTB request error")]
    Request(reqwest::Error),
}

const HLTB_URL_DOMAIN: &str = "https://howlongtobeat.com/";

#[derive(Debug)]
pub struct Game {
    pub title: String,
    pub hltb_url: String,
    // TODO actual time
    pub main_story_time: String,
    pub main_extra_time: String,
    pub completionist_time: String,
}

pub fn run(opt: &Opt) -> Result<Vec<Game>, Error> {
    let client = reqwest::Client::new();
    fetch_games(&client, &opt.game).and_then(parse_games)
}

fn fetch_games(client: &reqwest::Client, title: &str) -> Result<reqwest::Response, Error> {
    let params = [
        ("detail", ""),
        ("length_max", ""),
        ("length_min", ""),
        ("length_type", "main"),
        ("plat", ""),
        ("queryString", title),
        ("sortd", "Normal Order"),
        ("sorthead", "popular"),
        ("t", "games"),
    ];
    let url = format!("{}/{}", HLTB_URL_DOMAIN, "search_results.php?page=1");
    client
        .post(url.as_str())
        .form(&params)
        .send()
        .map_err(Error::Request)
}

fn parse_games(mut resp: reqwest::Response) -> Result<Vec<Game>, Error> {
    let resp_text = resp.text().map_err(Error::Request)?;
    let document = Document::from(resp_text.as_str());
    Ok(document
        .find(Name("li"))
        .map(parse_game)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect())
}

fn parse_game(node: Node) -> Option<Game> {
    let link_node = node.find(Name("a")).next();
    let title = link_node.and_then(|n| n.attr("title"))?;
    let link = format!(
        "{}{}",
        HLTB_URL_DOMAIN,
        link_node.and_then(|n| n.attr("href"))?
    );

    // There are three consecutive divs with same class for the main
    // story, main + extra, and completionist times.
    let mut time_divs = node.find(Class("center"));
    let main_story_time = time_divs
        .next()
        .map(|n| n.text())
        .unwrap_or_else(|| "".to_string());
    let main_extra_time = time_divs
        .next()
        .map(|n| n.text())
        .unwrap_or_else(|| "".to_string());
    let completionist_time = time_divs
        .next()
        .map(|n| n.text())
        .unwrap_or_else(|| "".to_string());

    Some(Game {
        title: String::from(title.to_owned().trim()),
        hltb_url: String::from(link.to_owned().trim()),
        main_story_time: String::from(main_story_time.trim()),
        main_extra_time: String::from(main_extra_time.trim()),
        completionist_time: String::from(completionist_time.trim()),
    })
}
