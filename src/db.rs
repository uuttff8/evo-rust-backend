use std::thread;
use std::time::Duration;

use crate::proposals::ProposalsRawData;
use crate::proposals::{ProposalJson, ProposalsJson};

use lazy_static::lazy_static;
use tokio_postgres::NoTls;

lazy_static! {
    static ref DB_ADRESS: &'static str = "host=localhost user=postgres";
}

pub async fn get_proposals() -> String {
    let (client, connection) = tokio_postgres::connect(*DB_ADRESS, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut body: String = "".into();
    for row in client
        .query("SELECT body FROM proposals", &[])
        .await
        .expect("SELECT body is not queried")
    {
        let value: String = row.get("body");
        body = value;
    }
    body
}

pub async fn generate_proposals() {
    tokio::spawn(async move {
        let (client, connection) = tokio_postgres::connect(*DB_ADRESS, NoTls).await.unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        loop {
            let props: String = process_props().unwrap().join("\n");
            dbg!(props.clone());
            let update_db = client
                .execute("UPDATE proposals SET body = $1 WHERE id = 1;", &[&props])
                .await;
            println!("{:?}", update_db);
            thread::sleep(Duration::from_secs(10));
        }
    });
}

fn process_props() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let proposals = ProposalsRawData::new()?.proposals;

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
