use async_openai::{Client, config::OpenAIConfig};
use reqwest::header::HeaderMap;

#[tokio::main]
async fn main() {
    let mut headers = HeaderMap::new();
    headers.insert("X-Cost-Center", "engineering".parse().unwrap());
    
    let req_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
        
    let config = OpenAIConfig::new();
    let _client = Client::with_config(config).with_http_client(req_client);
    println!("Compiled!");
}
