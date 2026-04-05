use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use factory_infrastructure::R2rClient;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct RetrieveContextTool {
    r2r_client: Arc<dyn R2rClient>,
}

impl RetrieveContextTool {
    pub fn new(r2r_client: Arc<dyn R2rClient>) -> Self {
        Self { r2r_client }
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

        match self.r2r_client.search(query).await {
            Ok(output) => Ok(CallToolResult {
                content: vec![McpContent::Text { text: output }],
                is_error: false,
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: format!("R2R search error: {}", e),
                }],
                is_error: true,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ManualMockR2rClient {
        should_fail: bool,
    }

    #[async_trait]
    impl R2rClient for ManualMockR2rClient {
        async fn search(&self, _query: &str) -> anyhow::Result<String> {
            if self.should_fail {
                Err(anyhow::anyhow!("R2R Failure"))
            } else {
                Ok("pattern content".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_retrieve_context_tool_success() {
        let mock_client = ManualMockR2rClient { should_fail: false };
        let tool = RetrieveContextTool::new(Arc::new(mock_client));
        let params = json!({ "query": "test query" });
        let result = tool.call(params).await.unwrap();

        assert!(!result.is_error);
        if let McpContent::Text { text } = &result.content[0] {
            assert_eq!(text, "pattern content");
        }
    }

    #[tokio::test]
    async fn test_retrieve_context_tool_failure() {
        let mock_client = ManualMockR2rClient { should_fail: true };
        let tool = RetrieveContextTool::new(Arc::new(mock_client));
        let params = json!({ "query": "test query" });
        let result = tool.call(params).await.unwrap();

        assert!(result.is_error);
        if let McpContent::Text { text } = &result.content[0] {
            assert!(text.contains("R2R search error: R2R Failure"));
        }
    }
}
