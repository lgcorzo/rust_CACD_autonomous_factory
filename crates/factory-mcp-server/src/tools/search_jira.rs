use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use factory_infrastructure::JiraClient;
use serde_json::{json, Value};
use std::sync::Arc;

pub struct SearchJiraTool {
    jira_client: Arc<dyn JiraClient>,
}

impl SearchJiraTool {
    pub fn new(jira_client: Arc<dyn JiraClient>) -> Self {
        Self { jira_client }
    }
}

#[async_trait]
impl Tool for SearchJiraTool {
    fn name(&self) -> String {
        "search_jira".to_string()
    }

    fn description(&self) -> String {
        "Searches Jira issues for summaries or descriptions.".to_string()
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

        match self.jira_client.search_issues(query).await {
            Ok(output) => Ok(CallToolResult {
                content: vec![McpContent::Text { text: output }],
                is_error: false,
            }),
            Err(e) => Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: format!("Jira search error: {}", e),
                }],
                is_error: true,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ManualMockJiraClient {
        should_fail: bool,
    }

    #[async_trait]
    impl JiraClient for ManualMockJiraClient {
        async fn search_issues(&self, _query: &str) -> anyhow::Result<String> {
            if self.should_fail {
                Err(anyhow::anyhow!("Internal Error"))
            } else {
                Ok("found it".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_search_jira_tool_success() {
        let mock_client = ManualMockJiraClient { should_fail: false };
        let tool = SearchJiraTool::new(Arc::new(mock_client));
        let params = json!({ "query": "test query" });
        let result = tool.call(params).await.unwrap();

        assert!(!result.is_error);
        if let McpContent::Text { text } = &result.content[0] {
            assert_eq!(text, "found it");
        }
    }

    #[tokio::test]
    async fn test_search_jira_tool_failure() {
        let mock_client = ManualMockJiraClient { should_fail: true };
        let tool = SearchJiraTool::new(Arc::new(mock_client));
        let params = json!({ "query": "test query" });
        let result = tool.call(params).await.unwrap();

        assert!(result.is_error);
        if let McpContent::Text { text } = &result.content[0] {
            assert!(text.contains("Jira search error: Internal Error"));
        }
    }
}
