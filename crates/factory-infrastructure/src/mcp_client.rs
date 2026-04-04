use serde::Serialize;
use serde_json::{json, Value};
use reqwest::Client;
use anyhow::{anyhow, Result};

pub struct McpHttpClient {
    client: Client,
    base_url: String,
}

impl McpHttpClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn call_tool<T: Serialize>(&self, name: &str, arguments: T) -> Result<Value> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "call_tool",
            "params": {
                "name": name,
                "arguments": arguments
            },
            "id": 1
        });

        let response = self.client
            .post(format!("{}/sse", self.base_url)) // Assuming SSE endpoint for now, or a generic relay
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("MCP server returned error: {}", response.status()));
        }

        let body: Value = response.json().await?;
        if let Some(error) = body.get("error") {
            return Err(anyhow!("MCP Tool Error: {}", error["message"]));
        }

        Ok(body["result"].clone())
    }
}
