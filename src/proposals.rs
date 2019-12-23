use scraper::{ElementRef, Html, Selector};
use selectors::attr::CaseSensitivity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct ProposalsJson {
    pub proposal: ProposalJson,
}

#[derive(Serialize, Deserialize)]
pub struct ProposalJson {
    pub title: String,
    pub id: String,
    pub date: String,
    pub issue: String,
}

pub struct ProposalsRawData {
    pub proposal: Vec<ProposalRawData>,
}

pub struct ProposalRawData {
    pub title: String,
    pub text_reference: String,
    pub text: Option<String>,
}

impl ProposalsRawData {
    pub fn new() -> Result<ProposalsRawData, Box<dyn Error>> {
        let mut rfcs: ProposalsRawData = ProposalsRawData {
            proposal: Vec::new(),
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

                        rfcs.proposal.push(ProposalRawData {
                            title: title,
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
            .skip_while(|x| (*x == ':') == false)
            .skip(2)
            .skip_while(|x| (*x == '(') == false)
            .skip(1)
            .take_while(|x| (*x == ')') == false)
            .collect();

        Ok(issue)
    }

    // TODO: not to use reqwest on each date
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
            .skip_while(|x| (*x == ':') == false)
            .skip(2)
            .collect();

        Ok(date)
    }

    pub fn get_id(&self) -> String {
        let valid_ref_n: String = self
            .text_reference
            .chars()
            .skip(33) // skip "/rust-lang/rfcs/blob/master/text/"
            .collect();

        let issue_number: String = valid_ref_n
            .chars()
            .take_while(|x| (*x == '-') == false)
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
            .proposal;
        prop.into_iter()
    }
}
