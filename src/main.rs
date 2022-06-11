use std::{collections::HashMap, path::PathBuf};

use clap::Parser;

#[allow(dead_code)]
static VALID_EXTENSIONS: [&str; 3] = [".js", ".script", ".ns"];

#[derive(Parser, Debug)]
struct Args {
    sync_dir: PathBuf,
}

fn main() {
    let _args = Args::parse();
    run();
}

#[tokio::main()]
async fn run() {
    send_file("q.ns", "SGVsbG8=").await;
}

async fn send_file(filename: impl Into<String>, body: impl Into<String>) {
    let mut file_data: HashMap<&'static str, String> = HashMap::new();
    file_data.insert("filename", filename.into());
    file_data.insert("code", body.into());

    let client = reqwest::Client::new();
    let res = client
        .put("http://localhost:9990/")
        .bearer_auth("")
        .json(&file_data)
        .send()
        .await
        .expect("failed to send file");
    println!("response: {:#?}", res);
    let body_json: serde_json::Value = res.json().await.expect("failed to receive response body");
    println!("body: {:#?}", body_json);
}
