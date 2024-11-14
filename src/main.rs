use csv::ReaderBuilder;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct SqlLog {
    conn_hash: String,
    stmt_id: u32,
    exec_id: u32,
    exec_time: String,
    sql_type: String,
    exe_status: String,
    db_ip: String,
    client_ip: String,
    client_host: String,
    app_name: String,
    db_user: String,
    sql_hash: String,
    from_tbs: String,
    select_cols: String,
    sql_stmt: String,
    stmt_bind_vars: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "sql_logs.tsv";
    let elastic_url = "http://localhost:9200/sql_logs/_doc";

    let client = Client::new();
    let mut rdr = ReaderBuilder::new().delimiter(b'\t').from_path(file_path)?;

    for result in rdr.deserialize() {
        let record: SqlLog = result?;
        let response = client.post(elastic_url).json(&record).send().await?;

        if response.status().is_success() {
            println!("Successfully sent record to Elasticsearch");
        } else {
            println!("Failed to send record to Elasticsearch");
        }
    }

    Ok(())
}
