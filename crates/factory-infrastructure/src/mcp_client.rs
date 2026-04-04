use serde::Serialize;
use serde_json::{json, Value};
use reqwest::Client;
use anyhow::{anyhow, Result};

#[cfg_attr(any(test, feature = "test-utils"), mockall::automock)]
#[async_trait::async_trait]
pub trait McpClient: Send + Sync {
    async fn call_tool_json(&self, name: &str, arguments: Value) -> anyhow::Result<Value>;
}

pub struct McpHttpClient {
    client: Client,
    base_url: String,
}

#[async_trait::async_trait]
impl McpClient for McpHttpClient {
    async fn call_tool_json(&self, name: &str, arguments: Value) -> anyhow::Result<Value> {
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
            .post(format!("{}/sse", self.base_url))
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

impl McpHttpClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[tokio::test]
    async fn test_call_tool_success() {
        let mock_server: MockServer = MockServer::start().await;
        let response_body = json!({
            "jsonrpc": "2.0",
            "result": { "output": "success" },
            "id": 1
        });

        Mock::given(method("POST"))
            .and(path("/sse"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = McpHttpClient::new(mock_server.uri());
        let result = client.call_tool_json("test_tool", json!({})).await.unwrap();
        
        assert_eq!(result["output"], "success");
    }

    #[tokio::test]
    async fn test_call_tool_error() {
        let mock_server: MockServer = MockServer::start().await;
        let response_body = json!({
            "jsonrpc": "2.0",
            "error": { "code": -32603, "message": "Tool failed" },
            "id": 1
        });

        Mock::given(method("POST"))
            .and(path("/sse"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = McpHttpClient::new(mock_server.uri());
        let result: anyhow::Result<Value> = client.call_tool_json("test_tool", json!({})).await;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "MCP Tool Error: Tool failed");
    }
}
