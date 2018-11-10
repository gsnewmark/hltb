#[macro_use]
extern crate failure;
#[macro_use]
extern crate structopt;

extern crate reqwest;

extern crate select;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

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
    title: String,
    hltb_url: String,
    // TODO actual time
    main_story_time: String,
    main_extra_time: String,
    completionist_time: String,
}

pub fn run(opt: &Opt) -> Result<Vec<Game>, Error> {
    let client = reqwest::Client::new();
    fetch_games(&client, &opt.game).and_then(parse_games)
}

fn fetch_games(client: &reqwest::Client, title: &String) -> Result<reqwest::Response, Error> {
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
    client
        .post("https://howlongtobeat.com/search_main.php?page=1")
        .form(&params)
        .send()
        .map_err(|e| Error::Request(e))
}

fn parse_games(mut resp: reqwest::Response) -> Result<Vec<Game>, Error> {
    let resp_text = resp.text().map_err(|e| Error::Request(e))?;
    let document = Document::from(resp_text.as_str());
    Ok(document.find(Name("li")).map(parse_game).collect())
}

fn parse_game(node: Node) -> Game {
    // TODO return Error instead of `unwrap`
    let link_node = node.find(Name("a")).next();
    let title = link_node.and_then(|n| n.attr("title")).unwrap();
    let link = format!(
        "{}{}",
        HLTB_URL_DOMAIN,
        link_node.and_then(|n| n.attr("href")).unwrap()
    );

    // There are three consecutive divs with same class for the main
    // story, main + extra, and completionist times.
    let mut time_divs = node.find(Class("center"));
    let main_story_time = time_divs.next().map(|n| n.text()).unwrap_or("".to_string());
    let main_extra_time = time_divs.next().map(|n| n.text()).unwrap_or("".to_string());
    let completionist_time = time_divs.next().map(|n| n.text()).unwrap_or("".to_string());

    Game {
        title: String::from(title.to_owned().trim()),
        hltb_url: String::from(link.to_owned().trim()),
        main_story_time: String::from(main_story_time.trim()),
        main_extra_time: String::from(main_extra_time.trim()),
        completionist_time: String::from(completionist_time.trim()),
    }
}
