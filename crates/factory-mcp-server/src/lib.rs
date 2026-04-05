pub mod protocol;
pub mod tools;

use axum::{extract::State, Json};
use crate::protocol::{JsonRpcRequest, JsonRpcResponse, McpTool};
use crate::tools::Tool;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct McpServer {
    tools: Arc<RwLock<HashMap<String, Box<dyn Tool>>>>,
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
        }
    }

    pub async fn add_tool(&self, tool: Box<dyn Tool>) {
        let mut tools = self.tools.write().await;
        tools.insert(tool.name(), tool);
    }

    pub async fn register_default_tools(&self) {
        use crate::tools::{
            execute_code::ExecuteCodeTool, plan_mission::PlanMissionTool,
            retrieve_context::RetrieveContextTool, security_review::SecurityReviewTool,
        };

        let litellm_api_key = std::env::var("LITELLM_API_KEY").unwrap_or_else(|_| "sk-placeholder".to_string());
        let litellm_base_url = std::env::var("LITELLM_BASE_URL").unwrap_or_else(|_| "http://litellm:4000/v1".to_string());
        let litellm_model = std::env::var("LITELLM_MODEL").unwrap_or_else(|_| "alibaba-cn/MiniMax/MiniMax-M2.7".to_string());
        let r2r_base_url = std::env::var("R2R_BASE_URL").unwrap_or_else(|_| "http://r2r:8000".to_string());

        self.add_tool(Box::new(ExecuteCodeTool)).await;
        self.add_tool(Box::new(PlanMissionTool::new(litellm_api_key, litellm_base_url, litellm_model))).await;
        self.add_tool(Box::new(RetrieveContextTool::new(r2r_base_url))).await;
        self.add_tool(Box::new(SecurityReviewTool)).await;
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
                    self.error_response(request.id, -32603, &format!("Tool execution error: {}", e))
                }
            }
        } else {
            self.error_response(request.id, -32602, "Tool not found")
        }
    }

    pub async fn post_handler(
        State(server): State<Arc<McpServer>>,
        Json(request): Json<JsonRpcRequest>,
    ) -> Json<JsonRpcResponse> {
        Json(server.handle_request(request).await)
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
