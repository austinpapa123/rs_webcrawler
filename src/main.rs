use reqwest::Client;
use scraper::{Html, Selector};
use futures::future::join_all;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get command line args: expect file path as first argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: web_crawler <path_to_urls.txt>");
        std::process::exit(1);
    }
    let file_path = &args[1];

    // read URLs line by line
    let urls = read_lines(file_path)?
        .filter_map(|line| line.ok()) // remove io::Error
        .filter(|line| !line.trim().is_empty()) // skip empty lines
        .collect::<Vec<String>>();

    println!("Found {} URLs to crawl.\n", urls.len());

    let client: Client = Client::new();

    // fire off all requests concurrently
    let futures = urls.into_iter().map(|url| {
        let client = client.clone();
        async move {
            match fetch_title(&client, &url).await {
                Ok(title) => println!("{} -> {}", url, title),
                Err(e) => eprintln!("Error fetching {}: {}", url, e),
            }
        }
    });

    join_all(futures).await;
    Ok(())
}

// helper to read file line by line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn fetch_title(client: &Client, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("h1").unwrap();
    Ok(document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(""))
        .unwrap_or_else(|| "[No title found]".to_string()))
}
