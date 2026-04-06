use anyhow::anyhow;
use reqwest::Client;
use serde_json::{json, Value};
use tokio::sync::OnceCell;

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

        let response = self
            .client
            .post(format!("{}/mcp", self.base_url))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("MCP server returned error: {}", response.status()));
        }

        let body: Value = response.json().await?;
        if let Some(error) = body.get("error") {
            let msg = error["message"].as_str().unwrap_or("Unknown error");
            return Err(anyhow!("MCP Tool Error: {}", msg));
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

/// A client that uses SSE handshake to find the session endpoint
pub struct McpSseClient {
    client: Client,
    base_url: String,
    session_url: OnceCell<String>,
}

impl McpSseClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            session_url: OnceCell::new(),
        }
    }

    async fn get_session_url(&self) -> anyhow::Result<String> {
        self.session_url
            .get_or_try_init(|| async {
                let sse_url = format!("{}/sse", self.base_url);
                let response = self.client.get(&sse_url).send().await?;

                if !response.status().is_success() {
                    return Err(anyhow!("Failed to connect to SSE: {}", response.status()));
                }

                // In a real implementation, we'd wrap this and parse the stream.
                // For simplicity, we expect the first "endpoint" event or just check the headers if applicable.
                // Standard MCP SSE returns the endpoint in an 'endpoint' event.
                // Here we'll look for the first message that looks like a URL.
                let mut stream = response.bytes_stream();
                use futures_util::StreamExt;

                while let Some(item) = stream.next().await {
                    let chunk = item?;
                    let text = String::from_utf8_lossy(&chunk);
                    for line in text.lines() {
                        if line.starts_with("event: endpoint") {
                            // The next line should be the data: URL
                        } else if line.starts_with("data: ") {
                            let rel_url = line.trim_start_matches("data: ").trim();
                            if rel_url.starts_with('/') {
                                return Ok(format!("{}{}", self.base_url, rel_url));
                            } else {
                                return Ok(rel_url.to_string());
                            }
                        }
                    }
                }

                Err(anyhow!("No session endpoint found in SSE stream"))
            })
            .await
            .cloned()
    }
}

#[async_trait::async_trait]
impl McpClient for McpSseClient {
    async fn call_tool_json(&self, name: &str, arguments: Value) -> anyhow::Result<Value> {
        let session_url = self.get_session_url().await?;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "call_tool",
            "params": {
                "name": name,
                "arguments": arguments
            },
            "id": 1
        });

        let response = self.client.post(&session_url).json(&request).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("MCP session returned error: {}", response.status()));
        }

        let body: Value = response.json().await?;
        if let Some(error) = body.get("error") {
            let msg = error["message"].as_str().unwrap_or("Unknown error");
            return Err(anyhow!("MCP Tool Error: {}", msg));
        }

        Ok(body["result"].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_call_tool_http_success() {
        let mock_server: MockServer = MockServer::start().await;
        let response_body = json!({
            "jsonrpc": "2.0",
            "result": { "output": "success" },
            "id": 1
        });

        Mock::given(method("POST"))
            .and(path("/mcp"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = McpHttpClient::new(mock_server.uri());
        let result = client.call_tool_json("test_tool", json!({})).await.unwrap();

        assert_eq!(result["output"], "success");
    }

    #[tokio::test]
    async fn test_call_tool_sse_success() {
        let mock_server: MockServer = MockServer::start().await;

        // Mock SSE endpoint
        Mock::given(method("GET"))
            .and(path("/sse"))
            .respond_with(ResponseTemplate::new(200).set_body_string(format!(
                "event: endpoint\ndata: {}/mcp/session/1\n\n",
                mock_server.uri()
            )))
            .mount(&mock_server)
            .await;

        // Mock Tool Session endpoint
        let response_body = json!({
            "jsonrpc": "2.0",
            "result": { "output": "sse_success" },
            "id": 1
        });

        Mock::given(method("POST"))
            .and(path("/mcp/session/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&mock_server)
            .await;

        let client = McpSseClient::new(mock_server.uri());
        let result = client.call_tool_json("test_tool", json!({})).await.unwrap();

        assert_eq!(result["output"], "sse_success");
    }
}
