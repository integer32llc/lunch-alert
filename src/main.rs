use scraper::{Html, Selector};

fn main() {
    let html = std::fs::read_to_string("2025-08-17.html").unwrap();
    let doc = Html::parse_document(&html);
    let selector = Selector::parse("main a").unwrap();
    let mut smoosh: Vec<(String, String)> = Vec::new();

    for element in doc.select(&selector) {
        smoosh.push((
            element.value().attr("href").unwrap().into(),
            element.value().attr("data-file-name").unwrap().into(),
        ));
    }

    smoosh.dedup();

    for (path, filename) in smoosh.into_iter().take(3) {
        println!("{filename}: https://www.pghschools.org{path}");
    }
}
