// use soup::prelude::*;
use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;
use std::error::Error;
// use scraper::node::Element;

fn main() -> Result<(), Box<dyn Error>> {
    let html = reqwest::get("https://github.com/rust-lang/rfcs/tree/master/text")?.text()?;

    let document = Html::parse_document(html.as_ref());
    // let selector = Selector::parse("tbody").unwrap();

    let mut rfcs_title = Vec::new();
    let mut rfcs_href = Vec::new();

    let selector_a = Selector::parse("a").unwrap();

    for a in document.select(&selector_a) {
        let a: ElementRef = a;

        if a.value()
            .has_class("js-navigation-open", CaseSensitivity::CaseSensitive)
        {
            println!("{:?}", a.value().attr("title"));
            println!("{:?}", a.value().attr("href"));

            
            rfcs_title.push(a.value().attr("title"));
            rfcs_href.push(a.value().attr("href"));
        }
    }

    dbg!(rfcs_href.len());
    dbg!(rfcs_title.len());

    Ok(())
}
