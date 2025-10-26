use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let urls = vec![
        "https://httpbin.org/html",
        "https://example.com",
        "https://www.rust-lang.org",
    ];

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for url in urls {
        let url = url.to_string();
        let results_clone = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let title = fetch_title(&url).unwrap_or_else(|e| format!("Error: {}", e));
            let mut res = results_clone.lock().unwrap();
            res.push((url, title));
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nResults:");
    for (url, title) in results.lock().unwrap().iter() {
        println!("{} -> {}", url, title);
    }
}

fn fetch_title(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = get(url)?.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("h1").unwrap();
    Ok(document
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<Vec<_>>().join(""))
        .unwrap_or_else(|| "[No title found]".to_string()))
}
