#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

mod proposals;

use crate::proposals::{ProposalJson, ProposalsJson};
use crate::proposals::{ProposalRawData, ProposalsRawData};

use std::error::Error;
use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;
use postgres::{Client, NoTls};
use rocket::{get, routes};

lazy_static! {
    static ref DB_ADRESS: &'static str = "host=localhost user=postgres";
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

fn generate_proposals() {
    let mut client = Client::connect(*DB_ADRESS, NoTls).unwrap();

    thread::spawn(move || loop {
        let props: String = process_props().unwrap().join("\n");
        let update_db = client.execute("UPDATE proposals SET body = $1 WHERE id = 1;", &[&props]);

        println!("{:?}", update_db);

        thread::sleep(Duration::from_secs(900));
    });
}

#[get("/proposal/<id>")]
fn get_proposal(id: String) -> String {
    let props = ProposalRawData::new(id).unwrap();
    format!("{}", serde_json::to_string_pretty(&props).unwrap())
}

#[get("/proposals")]
fn proposals() -> String {
    let mut client2 = Client::connect(*DB_ADRESS, NoTls).unwrap();
    let mut body: String = "".into();

    for row in client2.query("SELECT body FROM proposals", &[]).unwrap() {
        let value: String = row.get("body");
        body = value;
    }

    body
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() -> Result<(), Box<dyn Error>> {
    generate_proposals();

    rocket::ignite()
        .mount("/", routes![hello, proposals, get_proposal])
        .launch();

    Ok(())
}
