use futures::stream::{FuturesUnordered, StreamExt};
use trpl::{Html, Response};

pub fn run_app(args: Vec<String>) {
    if args.len() < 2 {
        eprintln!("Usage: {} <URL1> [URL2 ... URLn]", args.get(0).map(|s| s.as_str()).unwrap_or("program"));
        std::process::exit(2);
    }

    // run the async runtime and handle N URLs
    trpl::run(async move {
        let urls: Vec<String> = args.into_iter().skip(1).collect();

        // build an unordered set of futures and await the first one that finishes
        let mut futs = FuturesUnordered::new();
        for url in urls.into_iter() {
            futs.push(page_title(url));
        }

        if let Some((url, maybe_title)) = futs.next().await {
            println!("{url} returned first");
            match maybe_title {
                Some(title) => println!("Its page title is: '{title}'"),
                None => println!("Its title could not be parsed"),
            }
        } else {
            eprintln!("No URLs to process");
        }
    })
}

async fn page_title(url: String) -> (String, Option<String>) {
    let response: Response = trpl::get(&url).await;
    let response_text: String = response.text().await;
    let title: Option<String> = Html::parse(&response_text)
        .select_first("title")
        .map(|e| e.inner_html());
    (url, title)
}