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
    let client = Client::new("");
    client.send_file("q.ns", "hello world").await;
}

#[derive(Clone, Debug)]
struct Client {
    client: reqwest::Client,
    auth_token: String,
}

impl Client {
    pub fn new(auth_token: impl Into<String>) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            auth_token: auth_token.into(),
        }
    }

    async fn send_file(&self, filename: impl Into<String>, body: impl Into<String>) {
        let mut file_data: HashMap<&'static str, String> = HashMap::new();
        file_data.insert("filename", filename.into());
        file_data.insert("code", base64::encode(body.into()));

        let res = self
            .client
            .put("http://localhost:9990/")
            .bearer_auth(&self.auth_token)
            .json(&file_data)
            .send()
            .await
            .expect("failed to send file");
        println!("response: {:#?}", res);
        let body_json: serde_json::Value =
            res.json().await.expect("failed to receive response body");
        println!("body: {:#?}", body_json);
    }
}
