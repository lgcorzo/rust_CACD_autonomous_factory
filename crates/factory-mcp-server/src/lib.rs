pub mod protocol;
pub mod sandbox;
pub mod tools;

use crate::protocol::{JsonRpcRequest, JsonRpcResponse, McpTool};
use crate::tools::Tool;
use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
    Json,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_stream::{Stream, StreamExt};

pub struct McpServer {
    tools: Arc<RwLock<HashMap<String, Box<dyn Tool>>>>,
    sessions: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<JsonRpcResponse>>>>,
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl McpServer {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_tool(&self, tool: Box<dyn Tool>) {
        let mut tools = self.tools.write().await;
        tools.insert(tool.name(), tool);
    }
    pub async fn register_default_tools(&self) {
        use crate::tools::{
            execute_code::ExecuteCodeTool, index_code::IndexCodeTool,
            plan_mission::PlanMissionTool, retrieve_context::RetrieveContextTool,
            run_tests::RunTestsTool, search_jira::SearchJiraTool,
            security_review::SecurityReviewTool, update_mission_status::UpdateMissionStatusTool,
        };
        use factory_infrastructure::{HttpJiraClient, HttpR2rClient};

        let sandbox_driver = Arc::new(crate::sandbox::SubprocessDriver);
        let litellm_api_key =
            std::env::var("LITELLM_API_KEY").unwrap_or_else(|_| "sk-placeholder".to_string());
        let litellm_base_url = std::env::var("LITELLM_BASE_URL")
            .unwrap_or_else(|_| "http://litellm:4000/v1".to_string());
        let litellm_model = std::env::var("LITELLM_MODEL")
            .unwrap_or_else(|_| "alibaba-cn/MiniMax/MiniMax-M2.7".to_string());

        let r2r_base_url =
            std::env::var("R2R_BASE_URL").unwrap_or_else(|_| "http://r2r:8000".to_string());
        let r2r_user = std::env::var("R2R_USER").unwrap_or_else(|_| "admin".to_string());
        let r2r_pwd = std::env::var("R2R_PWD").unwrap_or_else(|_| "admin".to_string());
        let r2r_client = Arc::new(HttpR2rClient::new(r2r_base_url.clone(), r2r_user, r2r_pwd));

        let jira_url =
            std::env::var("JIRA_URL").unwrap_or_else(|_| "https://jira.example.com".to_string());
        let jira_user = std::env::var("JIRA_USERNAME").unwrap_or_else(|_| "user".to_string());
        let jira_token = std::env::var("JIRA_API_TOKEN").unwrap_or_else(|_| "token".to_string());
        let jira_client = Arc::new(HttpJiraClient::new(jira_url, jira_user, jira_token));

        self.add_tool(Box::new(ExecuteCodeTool::new(sandbox_driver.clone())))
            .await;
        self.add_tool(Box::new(PlanMissionTool::new(
            litellm_api_key,
            litellm_base_url,
            litellm_model,
        )))
        .await;
        self.add_tool(Box::new(RetrieveContextTool::new(r2r_client.clone())))
            .await;
        self.add_tool(Box::new(IndexCodeTool::new(r2r_base_url.clone())))
            .await;
        self.add_tool(Box::new(SearchJiraTool::new(jira_client)))
            .await;
        self.add_tool(Box::new(RunTestsTool::new(sandbox_driver)))
            .await;
        self.add_tool(Box::new(SecurityReviewTool)).await;
        self.add_tool(Box::new(UpdateMissionStatusTool::new("wiki".to_string())))
            .await;
    }

    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "list_tools" => self.handle_list_tools(request.id).await,
            "call_tool" => self.handle_call_tool(request).await,
            _ => self.error_response(request.id, -32601, "Method not found"),
        }
    }

    async fn handle_list_tools(&self, id: Option<Value>) -> JsonRpcResponse {
        let tools = self.tools.read().await;
        let mcp_tools: Vec<McpTool> = tools
            .values()
            .map(|t| McpTool {
                name: t.name(),
                description: t.description(),
                input_schema: t.input_schema(),
            })
            .collect();

        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({ "tools": mcp_tools })),
            error: None,
            id,
        }
    }

    async fn handle_call_tool(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        let params = request.params.unwrap_or(Value::Null);
        let name = params["name"].as_str().unwrap_or("");
        let tool_params = params["arguments"].clone();

        let tools = self.tools.read().await;
        if let Some(tool) = tools.get(name) {
            match tool.call(tool_params).await {
                Ok(result) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: Some(serde_json::to_value(result).unwrap()),
                    error: None,
                    id: request.id,
                },
                Err(e) => {
                    tracing::error!("Tool execution error for {}: {}", name, e);
                    self.error_response(request.id, -32603, "Tool execution error")
                }
            }
        } else {
            self.error_response(request.id, -32602, "Tool not found")
        }
    }

    pub async fn sse_handler(
        State(server): State<Arc<McpServer>>,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let (tx, rx) = mpsc::unbounded_channel::<JsonRpcResponse>();

        {
            let mut sessions = server.sessions.write().await;
            sessions.insert(session_id.clone(), tx);
        }

        tracing::info!("New SSE session established: {}", session_id);

        let stream = UnboundedReceiverStream::new(rx).map(|msg| {
            let json = serde_json::to_string(&msg).unwrap();
            Ok::<Event, Infallible>(Event::default().data(json))
        });

        // Send the initial endpoint event as per MCP spec
        let endpoint_url = format!("/mcp?session_id={}", session_id);
        let endpoint_event = Event::default().event("endpoint").data(endpoint_url);

        let initial_stream = tokio_stream::once(Ok(endpoint_event));
        let combined_stream = initial_stream.chain(stream);

        Sse::new(combined_stream).keep_alive(ax_keep_alive())
    }

    pub async fn post_handler(
        State(server): State<Arc<McpServer>>,
        Query(params): Query<HashMap<String, String>>,
        Json(request): Json<JsonRpcRequest>,
    ) -> Json<JsonRpcResponse> {
        let session_id = params.get("session_id");
        let response = server.handle_request(request).await;

        if let Some(sid) = session_id {
            let sessions = server.sessions.read().await;
            if let Some(tx) = sessions.get(sid) {
                let _ = tx.send(response.clone());
            }
        }

        Json(response)
    }

    fn error_response(&self, id: Option<Value>, code: i32, message: &str) -> JsonRpcResponse {
        JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(crate::protocol::JsonRpcError {
                code,
                message: message.to_string(),
                data: None,
            }),
            id,
        }
    }
}

fn ax_keep_alive() -> axum::response::sse::KeepAlive {
    axum::response::sse::KeepAlive::new()
        .interval(Duration::from_secs(15))
        .text("keep-alive")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::CallToolResult;
    use crate::tools::MockTool;

    #[tokio::test]
    async fn test_list_tools() {
        let server = McpServer::new();
        let mut mock_tool = MockTool::new();
        mock_tool
            .expect_name()
            .return_const("test_tool".to_string());
        mock_tool
            .expect_description()
            .return_const("A test tool".to_string());
        mock_tool.expect_input_schema().return_const(json!({}));

        server.add_tool(Box::new(mock_tool)).await;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "list_tools".to_string(),
            params: None,
            id: Some(json!(1)),
        };

        let response = server.handle_request(request).await;
        assert!(response.result.is_some());
        let result_val = response.result.unwrap();
        let tools = result_val["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0]["name"], "test_tool");
    }

    #[tokio::test]
    async fn test_call_tool_not_found() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "call_tool".to_string(),
            params: Some(json!({ "name": "unknown" })),
            id: Some(json!(1)),
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().message, "Tool not found");
    }

    #[tokio::test]
    async fn test_call_tool_error_sanitization() {
        let server = McpServer::new();
        let mut mock_tool = MockTool::new();
        mock_tool
            .expect_name()
            .return_const("test_tool".to_string());
        mock_tool.expect_call().returning(|_| {
            Err(anyhow::anyhow!("Sensitive internal error details"))
        });

        server.add_tool(Box::new(mock_tool)).await;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "call_tool".to_string(),
            params: Some(json!({ "name": "test_tool", "arguments": {} })),
            id: Some(json!(1)),
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_some());
        let err = response.error.unwrap();
        assert_eq!(err.message, "Tool execution error");
        assert!(!format!("{:?}", err.data).contains("Sensitive"));
    }

    #[tokio::test]
    async fn test_call_tool_success() {
        let server = McpServer::new();
        let mut mock_tool = MockTool::new();
        mock_tool
            .expect_name()
            .return_const("test_tool".to_string());
        mock_tool.expect_call().returning(|_| {
            Ok(CallToolResult {
                content: vec![],
                is_error: false,
            })
        });

        server.add_tool(Box::new(mock_tool)).await;

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "call_tool".to_string(),
            params: Some(json!({ "name": "test_tool", "arguments": {} })),
            id: Some(json!(1)),
        };

        let response = server.handle_request(request).await;
        assert!(response.result.is_some());
    }
}
