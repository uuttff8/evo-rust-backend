use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;
use std::error::Error;

struct ProposalRawData {
    title: String,
    text_reference: String,
}

impl ProposalRawData {
    // return vec because right now i have only vec
    pub fn new() -> Result<Vec<ProposalRawData>, Box<dyn Error>> {
        let mut rfcs: Vec<ProposalRawData> = Vec::new();

        let html = reqwest::get("https://github.com/rust-lang/rfcs/tree/master/text")?.text()?;
        let document = Html::parse_document(html.as_ref());

        let selector_a = Selector::parse("a").unwrap();

        for a in document.select(&selector_a) {
            let a: ElementRef = a;

            if a.value()
                .has_class("js-navigation-open", CaseSensitivity::CaseSensitive)
            {
                if let Some(pat) = a.value().attr("href") {
                    if pat != "" && pat != "/rust-lang/rfcs" {
                        let title = a.value().attr("title").expect("huiinya").into();
                        let href = a.value().attr("href").expect("what a hell").into();

                        rfcs.push(ProposalRawData {
                            title: title,
                            text_reference: href,
                        });
                    }
                }
            }
        }

        Ok(rfcs)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let proposals = ProposalRawData::new()?;

    dbg!(&proposals[0].title);
    dbg!(&proposals[0].text_reference);

    Ok(())
}
