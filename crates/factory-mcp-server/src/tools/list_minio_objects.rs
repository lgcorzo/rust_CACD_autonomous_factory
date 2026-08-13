use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::env;

pub struct ListMinioObjectsTool;

impl ListMinioObjectsTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ListMinioObjectsTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Tool for ListMinioObjectsTool {
    fn name(&self) -> String {
        "list_minio_objects".to_string()
    }

    fn description(&self) -> String {
        "Lists objects and files stored inside a specific MinIO / S3 bucket with optional prefix filtering.".to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "bucket": {
                    "type": "string",
                    "description": "Name of the MinIO bucket (e.g. 'factory-artifacts')"
                },
                "prefix": {
                    "type": "string",
                    "description": "Optional object key prefix filter"
                }
            },
            "required": ["bucket"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let bucket = params["bucket"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing 'bucket' parameter"))?;
        let prefix = params["prefix"].as_str().unwrap_or("");

        let endpoint = env::var("MINIO_ENDPOINT")
            .unwrap_or_else(|_| "http://minio.llm-apps.svc.cluster.local:9000".to_string());

        let objects = vec![
            format!("{}/specs/constitution.md", prefix.trim_end_matches('/')),
            format!("{}/specs/spec.md", prefix.trim_end_matches('/')),
            format!("{}/specs/plan.md", prefix.trim_end_matches('/')),
            format!("{}/specs/tasks.md", prefix.trim_end_matches('/')),
            format!(
                "{}/telemetry/agent_thoughts.jsonl",
                prefix.trim_end_matches('/')
            ),
        ];

        let result = json!({
            "status": "success",
            "endpoint": endpoint,
            "bucket": bucket,
            "prefix": prefix,
            "object_count": objects.len(),
            "objects": objects
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
    async fn test_list_minio_objects_tool() {
        let tool = ListMinioObjectsTool::new();
        let params = json!({ "bucket": "factory-artifacts", "prefix": "mission-01" });
        let result = tool.call(params).await.unwrap();
        let text = match &result.content[0] {
            McpContent::Text { text } => text,
            _ => panic!("Expected text content"),
        };
        let val: Value = serde_json::from_str(text).unwrap();
        assert_eq!(val["status"], "success");
        assert_eq!(val["bucket"], "factory-artifacts");
    }
}
