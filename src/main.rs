use csv::ReaderBuilder;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::error::Error;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct SqlLog {
    sql_hash: String,
    from_tbs: String,
    select_cols: String,
    sql_stmt: String,
    stmt_bind_vars: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "sql_logs.tsv";
    let elastic_url =
        env::var("ELASTIC_URL").expect("ELASTIC_URL must be set in environment variables");
    let elastic_url = format!("{}/sql_logs/_bulk", elastic_url);
    //let elastic_url = "http://127.0.0.1:9200/sql_logs/_bulk";

    //let client = Client::new();
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_path(file_path)?;

    let counter = Arc::new(AtomicUsize::new(0));
    let stdout_mutex = Arc::new(Mutex::new(()));

    process_records(&client, &mut rdr, &elastic_url, counter, stdout_mutex).await?;

    Ok(())
}

async fn process_records<R: std::io::Read>(
    client: &Client,
    rdr: &mut csv::Reader<R>,
    elastic_url: &str,
    counter: Arc<AtomicUsize>,
    stdout_mutex: Arc<Mutex<()>>,
) -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::channel(100);

    let client = client.clone();
    let elastic_url = elastic_url.to_string();

    tokio::spawn(async move {
        let mut bulk_records = Vec::new();
        while let Some(record) = rx.recv().await {
            bulk_records.push(record);
            if bulk_records.len() >= 1000 {
                if let Err(e) =
                    send_bulk_to_elasticsearch(&client, &elastic_url, &bulk_records).await
                {
                    eprintln!("Failed to send bulk records: {}", e);
                }
                bulk_records.clear();
            }
        }
        if !bulk_records.is_empty() {
            if let Err(e) = send_bulk_to_elasticsearch(&client, &elastic_url, &bulk_records).await {
                eprintln!("Failed to send bulk records: {}", e);
            }
        }
    });

    for result in rdr.deserialize() {
        let record: SqlLog = result?;
        tx.send(record).await?;
    }

    Ok(())
}

async fn send_bulk_to_elasticsearch(
    client: &Client,
    elastic_url: &str,
    records: &[SqlLog],
) -> Result<(), Box<dyn Error>> {
    let mut bulk_body = String::new();
    for record in records {
        let action = json!({ "index": {} });
        let data = serde_json::to_string(record)?;
        bulk_body.push_str(&format!("{}\n{}\n", action, data));
    }
    println!("Sending bulk request to: {}", elastic_url); // 添加这一行

    let response = client
        .post(elastic_url)
        .header("Content-Type", "application/x-ndjson")
        .body(bulk_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let text = response.text().await?;
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to send bulk request: {}", text),
        )));
    }

    Ok(())
}
