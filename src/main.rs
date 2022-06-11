use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use clap::Parser;
use walkdir::DirEntry;

#[derive(Parser, Debug)]
struct Args {
    sync_dir: PathBuf,
}

fn main() {
    let args = Args::parse();
    run(args);
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

static VALID_EXTENSIONS: [&str; 3] = [".js", ".script", ".ns"];

fn is_valid(entry: &DirEntry) -> bool {
    // Don't skip directories or walking won't happen as the root
    // directory will be skipped
    if entry.file_type().is_dir() {
        return true;
    }

    entry
        .file_name()
        .to_str()
        .map(|s| {
            dbg!(s);
            for ext in &VALID_EXTENSIONS {
                if s.ends_with(ext) {
                    dbg!(ext);
                    return true;
                }
            }
            false
        })
        .unwrap_or(false)
}

#[tokio::main()]
async fn run(args: Args) {
    let client = Client::new("", args.sync_dir.clone());

    let walker = walkdir::WalkDir::new(&args.sync_dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_hidden(e) && is_valid(e));
    for entry in walker {
        let entry = entry.expect("failed while walking directory");
        if entry.file_type().is_file() {
            println!("entry path {}", entry.path().display());
            client.send_file(entry.path(), "hello world").await;
        }
    }
}

#[derive(Clone, Debug)]
struct Client {
    client: reqwest::Client,
    auth_token: String,
    base_directory: PathBuf,
}

impl Client {
    pub fn new(auth_token: impl Into<String>, base_directory: PathBuf) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            auth_token: auth_token.into(),
            base_directory,
        }
    }

    async fn send_file(&self, filename: &Path, body: impl Into<String>) {
        let mut file_data: HashMap<&'static str, String> = HashMap::new();
        file_data.insert("filename", self.munge_filename(filename));
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

    fn munge_filename(&self, path: &Path) -> String {
        dbg!(&self.base_directory, path);
        let relative_path = path
            .strip_prefix(&self.base_directory)
            .expect("failed to strip prefix");

        relative_path.display().to_string()
    }
}
