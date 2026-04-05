use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct IndexCodeTool {
    r2r_base_url: String,
    http_client: reqwest::Client,
}

impl IndexCodeTool {
    pub fn new(r2r_base_url: String) -> Self {
        Self {
            r2r_base_url,
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Tool for IndexCodeTool {
    fn name(&self) -> String {
        "index_code".to_string()
    }

    fn description(&self) -> String {
        "Indexes a document into R2R for future retrieval.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "content": {"type": "string"},
                "metadata": {"type": "object"}
            },
            "required": ["content"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let content = params["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Content is required"))?;

        let metadata = params["metadata"].clone();

        let response = self
            .http_client
            .post(format!(
                "{}/v3/ingestion/ingest_documents",
                self.r2r_base_url
            ))
            .json(&json!({
                "documents": [{
                    "content": content,
                    "metadata": metadata,
                    "collection_id": "codebase"
                }]
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

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: "Document indexed successfully".to_string(),
            }],
            is_error: false,
        })
    }
}
