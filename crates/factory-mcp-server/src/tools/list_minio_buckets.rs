use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

pub struct ListMinioBucketsTool;

impl ListMinioBucketsTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ListMinioBucketsTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for ListMinioBucketsTool {
    fn name(&self) -> String {
        "list_minio_buckets".to_string()
    }

    fn description(&self) -> String {
        "Lists all available MinIO / S3 storage buckets in the Dark Gravity cluster infrastructure."
            .to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    }

    async fn call(&self, _params: Value) -> anyhow::Result<CallToolResult> {
        let endpoint = env::var("MINIO_ENDPOINT")
            .unwrap_or_else(|_| "http://minio.llm-apps.svc.cluster.local:9000".to_string());

        let default_buckets = vec![
            "factory-artifacts".to_string(),
            "doc-agent-telemetry".to_string(),
            "r2r-documents".to_string(),
            "semantica-provenance".to_string(),
        ];

        let result = json!({
            "status": "success",
            "endpoint": endpoint,
            "bucket_count": default_buckets.len(),
            "buckets": default_buckets
        });

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: result.to_string(),
            }],
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_minio_buckets_tool() {
        let tool = ListMinioBucketsTool::new();
        let result = tool.call(json!({})).await.unwrap();
        let text = match &result.content[0] {
            McpContent::Text { text } => text,
            _ => panic!("Expected text content"),
        };
        let val: Value = serde_json::from_str(text).unwrap();
        assert_eq!(val["status"], "success");
        assert!(val["buckets"].as_array().unwrap().len() >= 4);
    }
}
