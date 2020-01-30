use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;
use serde::{Deserialize, Serialize};
use std::error::Error;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ProposalsJson {
//     pub proposals: Vec<ProposalJson>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalsJson {
    pub proposals: Vec<ProposalJson>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProposalJson {
    pub title: String,
    pub index: String,
    pub date: String,
    pub issue: String,
    // pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalsRawData {
    pub proposals: Vec<ProposalRawData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProposalRawData {
    pub title: String,
    pub text_reference: String,
    pub text: Option<String>,
}

impl ProposalsRawData {
    pub fn new() -> Result<ProposalsRawData, Box<dyn Error>> {
        let mut rfcs: ProposalsRawData = ProposalsRawData {
            proposals: Vec::new(),
        };

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
                        let title: String = a.value().attr("title").expect("huiinya").into();
                        let href: String = a.value().attr("href").expect("what a hell").into();

                        rfcs.proposals.push(ProposalRawData {
                            title,
                            text_reference: href,
                            text: None,
                        });
                    }
                }
            }
        }

        Ok(rfcs)
    }
}

impl ProposalRawData {
    pub fn new(id: String) -> Result<ProposalRawData, Box<dyn Error>> {
        let mut prop = ProposalRawData {
            title: "".into(),
            text_reference: "".into(),
            text: None,
        };

        let html = reqwest::get("https://github.com/rust-lang/rfcs/tree/master/text")?.text()?;
        let document = Html::parse_document(html.as_ref());

        let selector_a = Selector::parse("a").unwrap();

        for a in document.select(&selector_a) {
            let a: ElementRef = a;

            if a.value()
                .has_class("js-navigation-open", CaseSensitivity::CaseSensitive)
            {
                if let Some(pat) = a.value().attr("title") {
                    if pat.contains(&id.to_string()) {
                        let href: String = a.value().attr("href").expect("what a hell").into();
                        let title: String = a.value().attr("href").expect("what a hell").into();

                        let valid_href = href.replace("/blob", "");
                        let text = format!("https://raw.githubusercontent.com{}", &valid_href);

                        let title = title.chars().skip(33).collect();
                        dbg!(&title);

                        prop = ProposalRawData {
                            title,
                            text_reference: href,
                            text: Some(text),
                        }
                    }
                }
            }
        }

        Ok(prop)
    }

    pub fn get_text(&mut self) -> Result<String, Box<dyn Error>> {
        let link = self.get_exact_link();
        let text = reqwest::get(link.as_str())?.text()?;

        self.text = Some(String::from(&text));

        Ok(text)
    }

    pub fn get_issue_link(&mut self) -> Result<String, Box<dyn Error>> {
        let issue_line: String = self
            .text
            .clone()
            .unwrap_or(self.get_text()?)
            .lines()
            .filter(|x| x.contains("Rust Issue"))
            .collect();

        let issue: String = issue_line
            .chars()
            .skip_while(|x| *x != ':')
            .skip(2)
            .skip_while(|x| *x != '(')
            .skip(1)
            .take_while(|x| *x != ')')
            .collect();

        Ok(issue)
    }

    pub fn get_date(&mut self) -> Result<String, Box<dyn Error>> {
        let date_line: String = self
            .text
            .clone()
            .unwrap_or(self.get_text()?)
            .lines()
            .filter(|x| x.contains("Start Date"))
            .collect();

        let date: String = date_line
            .chars()
            .skip_while(|x| *x != ':')
            .skip(2)
            .collect();

        Ok(date)
    }

    pub fn get_index(&self) -> String {
        let issue_number: String = self
            .text_reference
            .chars()
            .skip(33) // skip "/rust-lang/rfcs/blob/master/text/"
            .take_while(|x| *x != '-')
            .collect();

        issue_number
    }

    fn get_exact_link(&self) -> String {
        let valid_ref = self.text_reference.replace("/blob", "");
        format!("https://raw.githubusercontent.com{}", &valid_ref)
    }
}

impl IntoIterator for ProposalsRawData {
    type Item = ProposalRawData;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let prop = ProposalsRawData::new()
            .expect("Please check your internet connection!")
            .proposals;
        prop.into_iter()
    }
}
