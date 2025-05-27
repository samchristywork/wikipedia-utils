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

fn wiki_page(pageid: u64) -> String {
    let mut url = Url::parse(API_URL).expect("Failed to parse API URL");
    url.query_pairs_mut()
        .append_pair("action", "query")
        .append_pair("prop", "extracts")
        .append_pair("explaintext", "")
        .append_pair("exsectionformat", "plain")
        .append_pair("pageids", &pageid.to_string())
        .append_pair("format", "json");

    from_str::<Value>(
        &get(url.as_str())
            .expect("Failed to get URL")
            .text()
            .expect("Failed to get text from response"),
    )
    .expect("Failed to parse JSON")["query"]["pages"][pageid.to_string()]["extract"]
        .as_str()
        .expect("Expected extract to be a string")
        .to_string()
}

fn wiki_random(n: u64) -> Vec<String> {
    let mut url = Url::parse(API_URL).expect("Failed to parse API URL");
    url.query_pairs_mut()
        .append_pair("action", "query")
        .append_pair("list", "random")
        .append_pair("rnlimit", &n.to_string())
        .append_pair("rnnamespace", "0")
        .append_pair("format", "json");

    from_str::<Value>(
        &get(url.as_str())
            .expect("Failed to get URL")
            .text()
            .expect("Failed to get text from response"),
    )
    .expect("Failed to parse JSON")["query"]["random"]
        .as_array()
        .expect("Expected random to be an array")
        .iter()
        .map(|page| {
            format!(
                "{} ({})",
                page["title"]
                    .as_str()
                    .expect("Expected title to be a string")
                    .to_string(),
                page["id"].as_u64().expect("Expected id to be a u64")
            )
        })
        .collect()
}

fn usage() {
    println!(
        "Usage: wiki <command> [args]

Commands:
  search <term>  Search Wikipedia for a term
  page   <id>    Get the content of a Wikipedia page by ID
  random [n]     Get n random Wikipedia pages (default: 1)"
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
        "page" => {
            let pageid: u64 = std::env::args()
                .nth(2)
                .expect("No page ID provided")
                .parse()
                .expect("Invalid page ID");
            println!("{}", wiki_page(pageid));
        }
        "random" => {
            let n: u64 = std::env::args()
                .nth(2)
                .unwrap_or_else(|| "1".to_string())
                .parse()
                .expect("Invalid number of random pages");
            wiki_random(n).iter().for_each(|page| {
                println!("{}", page);
            });
        }
        _ => usage(),
    }
}
