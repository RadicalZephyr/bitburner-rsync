use std::collections::HashMap;

static VALID_EXTENSIONS: [&str; 3] = [".js", ".script", ".ns"];

// Connect to http://localhost:9990
// postURI: '/'
#[tokio::main()]
async fn main() {
    let mut file_data: HashMap<&'static str, &'static str> = HashMap::new();
    file_data.insert("filename", "file.script");
    file_data.insert("code", "SGVsbG8=");

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
