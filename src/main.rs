use reqwest::Client;
use scraper::{Html, Selector};
use futures::future::join_all;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://httpbin.org/html",
        "https://example.com",
        "https://www.rust-lang.org",
    ];

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
