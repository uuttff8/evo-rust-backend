#![feature(async_closure)]

mod proposals;

use crate::proposals::{ProposalJson, ProposalsJson};
use crate::proposals::{ProposalRawData, ProposalsRawData};

use std::thread;
use std::time::Duration;

use actix_web::{web, App, HttpServer, Responder};
use lazy_static::lazy_static;
use postgres::{Client, NoTls};

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

async fn generate_proposals() {
    thread::spawn(async move || {
        let mut client = Client::connect(*DB_ADRESS, NoTls).unwrap();

        loop {
            let props: String = process_props().unwrap().join("\n");
            let update_db =
                client.execute("UPDATE proposals SET body = $1 WHERE id = 1;", &[&props]);
            println!("{:?}", update_db);

            thread::sleep(Duration::from_secs(900));
        }
    });
}

async fn get_proposal(info: web::Path<String>) -> impl Responder {
    let props = ProposalRawData::new(info.into_inner()).unwrap();
    format!("{}", serde_json::to_string_pretty(&props).unwrap())
}

async fn get_proposals() -> impl Responder {
    let thread: String = thread::spawn(move || {
        let mut client = Client::connect(*DB_ADRESS, NoTls).unwrap();
        let mut body: String = "".into();
        for row in client
            .query("SELECT body FROM proposals", &[])
            .expect("SELECT body is not queried")
        {
            let value: String = row.get("body");
            body = value;
        }

        body
    })
    .join()
    .unwrap();

    thread
}

async fn index() -> impl Responder {
    format!("Hello")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    generate_proposals().await;

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/proposals").route(web::get().to(get_proposals)))
            .service(web::resource("/proposal/{name}").route(web::get().to(get_proposal)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
