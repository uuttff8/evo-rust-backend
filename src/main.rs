#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

mod proposals;

use crate::proposals::ProposalsRawData;
use crate::proposals::{ProposalJson, ProposalsJson};

use lazy_static::lazy_static;

use std::thread::sleep;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use rocket::get;
use rocket::response::Responder;
use rocket::{response, routes, Request, Response};

lazy_static! {
    static ref CACHE: ProposalCache = ProposalCache::new();
}

#[derive(Debug, Default)]
pub struct ProposalCache {
    pub proposals: Mutex<Vec<String>>,
    pub cache_timeout: Duration,
    pub now_time: Duration,
}

impl ProposalCache {
    pub fn new() -> Self {
        ProposalCache {
            proposals: Mutex::new(Vec::new()),
            cache_timeout: Duration::from_secs(900),
            now_time: Instant::now().elapsed(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.now_time > self.cache_timeout
    }
}

fn process_props() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let proposals = ProposalsRawData::new()?.proposal;

    let mut prop_array = Vec::<String>::new();

    for mut prop in proposals {
        let prop_json = ProposalJson {
            title: prop.title.to_string(),
            id: prop.get_id().to_string(),
            date: prop.get_date()?.to_string(),
            issue: prop.get_issue_link()?.to_string(),
        };

        let props_json = ProposalsJson {
            proposal: prop_json,
        };

        let j = serde_json::to_string(&props_json)?;

        prop_array.push(j);
    }

    Ok(prop_array)
}

fn cache_props() {
    loop {
        if CACHE.is_expired() {
            let props = process_props().expect("Error: Connection is down");

            let mut data = CACHE.proposals.lock().unwrap();
            *data = props;

            sleep(CACHE.cache_timeout);
        }
    }
}

#[get("/proposals")]
fn hello_bi() -> String {
    if !CACHE.is_expired() {
        CACHE.proposals.lock().unwrap().join("\n")
    } else {
        let net_props = process_props().expect("Error: Connection is down");
        net_props.join("\n")
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {

    std::thread::spawn(move || {
        cache_props();
    });

    rocket::ignite()
        .mount("/", routes![hello, hello_bi])
        .launch();
}
