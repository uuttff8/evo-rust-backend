mod proposals;

use crate::proposals::ProposalsRawData;
use crate::proposals::{ProposalJson, ProposalsJson};

use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use json::JsonValue;

use serde_json;

fn lol() -> Result<(), Box<dyn std::error::Error>> {
    let proposals = ProposalsRawData::new()?.proposal;

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
    }

    Ok(())
}

async fn index_proposals() -> Result<HttpResponse, Error> {
    let js: String = "asd".into();

    Ok(HttpResponse::Ok().json(js))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/proposals")
                .route(web::post().to(index_proposals))
            )
    })
    .bind("127.0.0.1:8080")?
    .start()
    .await
}

