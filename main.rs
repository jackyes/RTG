use std::thread;
use std::env;
use std::collections::HashSet;
use std::time::Duration;
use select::document::Document;
use select::predicate::Name;
use reqwest::blocking::Client;

static CONFIG_DEPTH: i32 = 6;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <url>", args[0]);
        return;
    }
    let uri = &args[1];
    let client = Client::new();
    let origin_url = uri.as_str();
    let res = client.get(origin_url).send();

    match res {
        Ok(res) => {
            println!("Status for {}: {}", origin_url, res.status());
            let textwp = res.text().expect("response text");
            let found_urls = Document::from(textwp.as_str())
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .map(String::from)
                .collect::<HashSet<String>>();
            for lk in &found_urls {
                if lk.contains("https") && !lk.contains(origin_url) {
                    visit_page(lk, 1);
                }
            }
        },
        Err(err) => {
            eprintln!("{}", err);
        },
    }
}

fn visit_page(uri: &str, mut depth: i32) {
    let client = Client::new();
    let origin_url = uri;
    let res = client.get(origin_url).send();

    match res {
        Ok(res) => {
            println!("Status for {}: {}, depth: {}", origin_url, res.status(), depth);
            let textwp = res.text().expect("response text");
            let found_urls = Document::from(textwp.as_str())
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .map(String::from)
                .collect::<HashSet<String>>();
            println!("Found {} links.", found_urls.len());

            for lk in &found_urls {
                if lk.contains("https") && !lk.contains(origin_url) && depth < CONFIG_DEPTH {
                    depth += 1;
                    visit_page(lk, depth);
                    thread::sleep(Duration::from_millis(400));
                } else if depth >= CONFIG_DEPTH {
                    println!("Depth {}. Stop.", CONFIG_DEPTH);
                    break;
                }
            }
        },
        Err(err) => {
            eprintln!("{}", err);
        },
    }
}
