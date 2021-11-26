use std::env;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::{thread, time::Duration};

static CONFIG_DEPTH: i32 = 6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _uri = &args[1].to_string();
    let client = reqwest::blocking::Client::new();
    let origin_url = &_uri;
    let  res = client.get(&**origin_url).send();
    match res {
        Ok(res) => {
            println!("Status for {}: {}", origin_url, res.status());
            let textwp = res.text().expect("response text");
            let found_urls = Document::from(textwp.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(str::to_string)
            .collect::<HashSet<String>>();
            
            for lk in &found_urls {
                if !&lk.contains(&**origin_url) && lk.contains("https") {
                    visit_page(lk,1)
                }
            }
        },
        Err(err) => {println!("{}",err)},
    }
}

fn visit_page(uri: &str, mut depth: i32){
    let client = reqwest::blocking::Client::new();
    let origin_url = uri;
    let res = client.get(origin_url).send();
    match res {
        Ok(res) => {

            println!("Status for {}: {}, depth: {}", origin_url, res.status(), depth);
            
            let textwp = res.text().expect("response text");
            let found_urls = Document::from(textwp.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(str::to_string)
            .collect::<HashSet<String>>();
            
            println!("Fond {} links.", &found_urls.len());
        
            for lk in &found_urls {
                if lk.contains("https") && depth <= CONFIG_DEPTH - 1 {
                    depth += 1;
                    visit_page(lk,depth);
                    thread::sleep(Duration::from_millis(400));           
                } else if depth >= CONFIG_DEPTH - 1 {
                    println!("depth {}. Stop.", CONFIG_DEPTH);
                    break;
                }
            }
    },
        Err(err) => {println!("{}",err)},
    }
}


