use reqwest::blocking::get;
use serde_json::Value;
use serde_json::from_str;
use url::Url;
use url::form_urlencoded::byte_serialize;

struct Page {
    title: String,
    pageid: u64,
}

const API_URL: &str = "https://en.wikipedia.org/w/api.php";

fn wiki_search(search_term: &str) -> Vec<Page> {
    let mut url = Url::parse(API_URL).expect("Failed to parse API URL");
    url.query_pairs_mut()
        .append_pair("action", "query")
        .append_pair("list", "search")
        .append_pair(
            "srsearch",
            &byte_serialize(search_term.as_bytes()).collect::<String>(),
        )
        .append_pair("format", "json");

    from_str::<Value>(
        &get(url.as_str())
            .expect("Failed to get URL")
            .text()
            .expect("Failed to get text from response"),
    )
    .expect("Failed to parse JSON")["query"]["search"]
        .as_array()
        .expect("Expected search results to be an array")
        .into_iter()
        .map(|result| Page {
            title: result["title"]
                .as_str()
                .expect("Expected title to be a string")
                .to_string(),
            pageid: result["pageid"]
                .as_u64()
                .expect("Expected pageid to be a u64"),
        })
        .collect()
}

fn usage() {
    println!(
        "Usage: wiki <command> [args]

Commands:
  search <term>  Search Wikipedia for a term"
    );
}

fn main() {
    let command = std::env::args().nth(1).unwrap_or_else(|| {
        usage();
        std::process::exit(1);
    });

    match command.as_str() {
        "search" => {
            let search_term = std::env::args().nth(2).expect("No search term provided");
            wiki_search(&search_term).iter().for_each(|page| {
                println!("{} ({})", page.title, page.pageid);
            });
        }
        _ => usage(),
    }
}
