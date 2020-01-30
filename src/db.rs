use std::time::Duration;

use crate::proposals::ProposalsRawData;
use crate::proposals::{ProposalJson, ProposalsJson};

use lazy_static::lazy_static;
use tokio_postgres::row::Row;
use tokio_postgres::NoTls;

lazy_static! {
    static ref DB_ADRESS: &'static str = "host=localhost user=postgres";
}

pub async fn get_proposals() -> String {
    let (client, connection) = tokio_postgres::connect(*DB_ADRESS, NoTls)
        .await
        .expect("Connection can not be made");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows: Vec<Row> = client
        .query("SELECT * FROM proposals", &[])
        .await
        .expect("SELECT all from db is not queried");

    let mut props_json = ProposalsJson {
        proposals: Vec::new(),
    };

    for row in rows {
        let prop_json = ProposalJson {
            title: row.get("title"),
            date: row.get("date"),
            index: row.get("index"),
            issue: row.get("issue"),
        };

        props_json.proposals.push(prop_json);
    }

    serde_json::to_string_pretty(&props_json).unwrap()
}

pub async fn generate_proposals() {
    tokio::spawn(async move {
        let (client, connection) = tokio_postgres::connect(*DB_ADRESS, NoTls)
            .await
            .expect("Connection can not be made");

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        loop {
            let props: Vec<ProposalJson> = process_props().unwrap();

            let _ = client
                .execute("DELETE FROM proposals WHERE id > 0;", &[])
                .await;

            for prop in props {
                let update_db = client
                .execute(
                    "INSERT INTO proposals (title, index, date, issue) VALUES ($1, $2, $3, $4);",
                    &[&prop.title, &prop.index, &prop.date, &prop.issue],
                )
                .await
                .unwrap();

                dbg!(update_db);
            }

            tokio::time::delay_for(Duration::from_secs(900)).await;
        }
    });
}

fn process_props() -> Result<Vec<ProposalJson>, Box<dyn std::error::Error>> {
    let proposals = ProposalsRawData::new()?;

    let mut props_json = Vec::new();

    for mut prop in proposals {
        // VERY expensive func
        let prop_json = ProposalJson {
            title: prop.title.to_string(),
            index: prop.get_index().to_string(),
            date: prop.get_date()?.to_string(),
            issue: prop.get_issue_link()?.to_string(),
        };

        props_json.push(prop_json);
    }

    Ok(props_json)
}
