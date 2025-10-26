use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() {
    // list of urLs to crawl
    let urls = vec![
        "https://httpbin.org/html",
        "https://example.com",
    ];

    for url in urls {
        println!("Fetching: {}", url);
        match fetch_title(url) {
            Ok(title) => println!("Title: {}\n", title),
            Err(e) => eprintln!("Error fetching {}: {}\n", url, e),
        }
    }
}

fn fetch_title(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // fetch the page content
    let body = get(url)?.text()?;

    // parse HTML using scraper crate
    let document = Html::parse_document(&body);
    let selector = Selector::parse("h1").unwrap();

    // extract the <title> text
    if let Some(element) = document.select(&selector).next() {
        let title = element.text().collect::<Vec<_>>().join("");
        Ok(title)
    } else {
        Ok("[No title found]".to_string())
    }
}
