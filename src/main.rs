use scraper::{Html, Selector};
use std::io;

fn main() {
    let stdin = io::stdin();
    let html = io::read_to_string(stdin).unwrap();
    let doc = Html::parse_document(&html);
    let selector = Selector::parse("main a").unwrap();
    let mut smoosh: Vec<(String, String)> = Vec::new();

    for element in doc.select(&selector) {
        if let Some(href) = element.value().attr("href")
            && let Some(file_name) = element.value().attr("data-file-name")
        {
            smoosh.push((href.into(), file_name.into()));
        }
    }

    smoosh.dedup();

    for (path, filename) in smoosh.into_iter().take(3) {
        println!("{filename}: https://www.pghschools.org{path}");
    }
}
