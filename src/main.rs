use trpl::{Either, Html, Response};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title1 = page_title(&args[1]);
        let title2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::race(title1, title2).await {
            trpl::Either::Left(left) => left,
            trpl::Either::Right(right) => right,
        };

        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("It's page title is: '{title}'"),
            None => println!("It's title could not be parsed"),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response: Response = trpl::get(url).await;
    let response_text: String = response.text().await;
    let title: Option<String> = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}
