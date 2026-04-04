pub mod protocol;
pub mod tools;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::{json, Value};
use crate::protocol::{JsonRpcRequest, JsonRpcResponse, McpTool};
use crate::tools::Tool;

pub struct McpServer {
    tools: Arc<RwLock<HashMap<String, Box<dyn Tool>>>>,
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

    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "list_tools" => self.handle_list_tools(request.id).await,
            "call_tool" => self.handle_call_tool(request).await,
            _ => self.error_response(request.id, -32601, "Method not found"),
        }
    }

    async fn handle_list_tools(&self, id: Option<Value>) -> JsonRpcResponse {
        let tools = self.tools.read().await;
        let mcp_tools: Vec<McpTool> = tools.values()
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
                Err(e) => self.error_response(request.id, -32603, &format!("Tool execution error: {}", e)),
            }
        } else {
            self.error_response(request.id, -32602, "Tool not found")
        }
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
