#![feature(async_closure)]

mod db;
mod proposals;

use crate::proposals::ProposalRawData;

use actix_web::{web, App, HttpServer, Responder};

async fn get_proposal(info: web::Path<String>) -> impl Responder {
    let props = ProposalRawData::new(info.into_inner()).unwrap();
    format!("{}", serde_json::to_string_pretty(&props).unwrap())
}

async fn get_proposals() -> impl Responder {
    db::get_proposals().await
}

async fn index() -> impl Responder {
    format!("Hello")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    db::generate_proposals().await;

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
