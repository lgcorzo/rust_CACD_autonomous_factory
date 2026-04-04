use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct RetrieveContextTool {
    r2r_base_url: String,
    http_client: reqwest::Client,
}

impl RetrieveContextTool {
    pub fn new(r2r_base_url: String) -> Self {
        Self {
            r2r_base_url,
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Tool for RetrieveContextTool {
    fn name(&self) -> String {
        "retrieve_context".to_string()
    }

    fn description(&self) -> String {
        "Queries R2R RAG for patterns.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string"}
            },
            "required": ["query"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let query = params["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Query is required"))?;

        let response = self
            .http_client
            .post(format!("{}/v3/retrieval/search", self.r2r_base_url))
            .json(&json!({
                "query": query,
                "search_settings": {
                    "use_vector_search": true,
                    "filters": {},
                    "limit": 5
                }
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: format!("R2R API error: {}", response.status()),
                }],
                is_error: true,
            });
        }

        let data: Value = response.json().await?;
        let results = data["results"]["chunk_search_results"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid response from R2R"))?;

        let mut output = String::new();
        for doc in results {
            let text = doc["text"].as_str().unwrap_or("");
            let score = doc["score"].as_f64().unwrap_or(0.0);
            output.push_str(&format!("[Score: {:.2}] {}\n\n", score, text));
        }

        Ok(CallToolResult {
            content: vec![McpContent::Text { text: output }],
            is_error: false,
        })
    }
}
