pub mod feedback_route;
pub mod github_webhook;
pub mod protocol;

pub mod sandbox;
pub mod skills;
pub mod tools;

use crate::github_webhook::{
    DefaultLocalMissionDispatcher, MissionDispatchReceipt, WebhookMissionDispatcher,
};
use crate::protocol::{JsonRpcRequest, JsonRpcResponse, McpTool};
use crate::tools::Tool;
use axum::{
    extract::{Query, Request, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{
        sse::{Event, Sse},
        IntoResponse, Response,
    },
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};

pub struct SessionRecord {
    pub owner_id: String,
    pub created_at: Instant,
    pub last_active: Instant,
    pub tx: mpsc::Sender<JsonRpcResponse>,
}

struct SessionDisconnectGuard<S> {
    inner: S,
    session_id: String,
    sessions: Arc<RwLock<HashMap<String, SessionRecord>>>,
}

impl<S, Item> Stream for SessionDisconnectGuard<S>
where
    S: Stream<Item = Item> + Unpin,
{
    type Item = Item;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

impl<S> Drop for SessionDisconnectGuard<S> {
    fn drop(&mut self) {
        let sid = self.session_id.clone();
        if let Ok(mut guard) = self.sessions.try_write() {
            guard.remove(&sid);
        } else {
            let sessions = self.sessions.clone();
            tokio::spawn(async move {
                sessions.write().await.remove(&sid);
            });
        }
    }
}

pub struct McpServer {
    tools: Arc<RwLock<HashMap<String, Box<dyn Tool>>>>,
    sessions: Arc<RwLock<HashMap<String, SessionRecord>>>,
    protected_mode: bool,
    api_bearer_token: Option<String>,
    webhook_secret: Option<String>,
    max_webhook_payload_bytes: usize,
    seen_webhook_deliveries: Arc<RwLock<HashSet<String>>>,
    webhook_dispatcher: Arc<dyn WebhookMissionDispatcher>,
    max_sessions: usize,
    session_ttl: Duration,
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl McpServer {
    pub fn new() -> Self {
        let is_prod = std::env::var("FACTORY_ENV")
            .unwrap_or_default()
            .eq_ignore_ascii_case("production")
            || cfg!(feature = "production");
        let token = std::env::var("MCP_API_BEARER_TOKEN")
            .ok()
            .filter(|t| !t.trim().is_empty());

        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            protected_mode: is_prod || token.is_some(),
            api_bearer_token: token,
            webhook_secret: None,
            max_webhook_payload_bytes: 256 * 1024,
            seen_webhook_deliveries: Arc::new(RwLock::new(HashSet::new())),
            webhook_dispatcher: Arc::new(DefaultLocalMissionDispatcher),
            max_sessions: 64,
            session_ttl: Duration::from_secs(300),
        }
    }

    pub fn with_protected_mode(mut self, protected: bool) -> Self {
        self.protected_mode = protected;
        self
    }

    pub fn with_api_bearer_token(mut self, token: String) -> Self {
        self.api_bearer_token = Some(token);
        self
    }

    pub fn with_webhook_secret(mut self, secret: String) -> Self {
        self.webhook_secret = Some(secret);
        self
    }

    pub fn with_max_webhook_payload_bytes(mut self, max_bytes: usize) -> Self {
        self.max_webhook_payload_bytes = max_bytes;
        self
    }

    pub fn with_webhook_dispatcher(
        mut self,
        dispatcher: Arc<dyn WebhookMissionDispatcher>,
    ) -> Self {
        self.webhook_dispatcher = dispatcher;
        self
    }

    pub fn with_max_sessions(mut self, max_sessions: usize) -> Self {
        self.max_sessions = max_sessions;
        self
    }

    pub fn with_session_ttl(mut self, ttl: Duration) -> Self {
        self.session_ttl = ttl;
        self
    }

    pub fn is_protected_mode(&self) -> bool {
        self.protected_mode
    }

    pub fn webhook_secret(&self) -> Option<String> {
        self.webhook_secret.clone()
    }

    pub fn max_webhook_payload_bytes(&self) -> usize {
        self.max_webhook_payload_bytes
    }

    pub async fn record_webhook_delivery(&self, delivery_id: &str) -> bool {
        let mut seen = self.seen_webhook_deliveries.write().await;
        if seen.contains(delivery_id) {
            return false;
        }
        seen.insert(delivery_id.to_string());
        true
    }

    pub async fn dispatch_webhook_mission(
        &self,
        delivery_id: &str,
        mission_id: &str,
        repository: &str,
        goal: &str,
    ) -> Result<MissionDispatchReceipt, String> {
        self.webhook_dispatcher
            .dispatch_webhook_mission(delivery_id, mission_id, repository, goal)
            .await
    }

    pub async fn active_session_count(&self) -> usize {
        self.prune_expired_and_closed_sessions().await;
        self.sessions.read().await.len()
    }

    pub async fn prune_expired_and_closed_sessions(&self) {
        let now = Instant::now();
        let ttl = self.session_ttl;
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, rec| !rec.tx.is_closed() && now.duration_since(rec.last_active) < ttl);
    }

    pub fn verify_caller_authorized(&self, headers: &HeaderMap) -> bool {
        if !self.protected_mode {
            return true;
        }
        let Some(expected) = self.api_bearer_token.as_deref() else {
            return false;
        };
        if expected.trim().is_empty() {
            return false;
        }
        let Some(auth_val) = headers.get("authorization").and_then(|v| v.to_str().ok()) else {
            return false;
        };
        let Some(provided) = auth_val.strip_prefix("Bearer ") else {
            return false;
        };
        provided.trim() == expected
    }

    fn resolve_owner_id(&self, headers: &HeaderMap) -> String {
        headers
            .get("x-mcp-client-id")
            .and_then(|v| v.to_str().ok())
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "default-authenticated-caller".to_string())
    }

    pub async fn add_tool(&self, tool: Box<dyn Tool>) {
        let mut tools = self.tools.write().await;
        tools.insert(tool.name(), tool);
    }

    pub async fn register_default_tools(&self) -> anyhow::Result<()> {
        use crate::tools::{
            bridge::BridgeTool, deep_research_tool::DeepResearchTool,
            execute_code::ExecuteCodeTool, index_code::IndexCodeTool,
            launch_sandbox_pod::LaunchSandboxPodTool, plan_mission::PlanMissionTool,
            retrieve_context::RetrieveContextTool, run_tests::RunTestsTool,
            search_jira::SearchJiraTool, security_review::SecurityReviewTool,
            spec_kit_tasks_to_issues::SpecKitTasksToIssuesTool, spec_kit_tool::SpecKitTool,
            update_mission_status::UpdateMissionStatusTool,
        };
        #[cfg(not(feature = "production"))]
        use factory_infrastructure::SimpleMockKafkaClient;
        use factory_infrastructure::{
            HttpGitlabClient, HttpJiraClient, HttpR2rClient, KafkaClient,
        };

        // Zero-Trust Default: Default to GvisorK8sDriver for all environments unless explicitly requested otherwise
        let sandbox_mode =
            std::env::var("SANDBOX_MODE").unwrap_or_else(|_| "gvisork8s".to_string());
        let sandbox_driver: Arc<dyn crate::sandbox::SandboxDriver> =
            if sandbox_mode.to_lowercase() == "subprocess" {
                Arc::new(crate::sandbox::SubprocessDriver::new())
            } else {
                Arc::new(crate::sandbox::GvisorK8sDriver)
            };

        let specify_cli_path =
            std::env::var("SPECIFY_CLI_PATH").unwrap_or_else(|_| "specify".to_string());
        let litellm_api_key =
            std::env::var("LITELLM_API_KEY").unwrap_or_else(|_| "sk-dummy".to_string());
        let litellm_base_url = std::env::var("LITELLM_BASE_URL")
            .or_else(|_| std::env::var("LITELLM_API_BASE"))
            .unwrap_or_else(|_| "http://litellm.llm-apps.svc.cluster.local:4000".to_string());

        let model_config = factory_core::AgentModelConfig::load();
        let planner_model = model_config.get_planner_model().to_string();
        let security_model = model_config.get_model("security_review").to_string();

        let finops_tag = factory_core::FinOpsTag {
            team: std::env::var("FINOPS_TEAM").unwrap_or_else(|_| "dark-gravity-ops".to_string()),
            epic: std::env::var("FINOPS_EPIC").unwrap_or_else(|_| "E6.3".to_string()),
            microservice: std::env::var("FINOPS_MICROSERVICE")
                .unwrap_or_else(|_| "factory-mcp-server".to_string()),
            environment: std::env::var("FINOPS_ENVIRONMENT")
                .unwrap_or_else(|_| "staging".to_string()),
            cost_center: std::env::var("FINOPS_COST_CENTER")
                .unwrap_or_else(|_| "engineering".to_string()),
        };

        let r2r_base_url = std::env::var("R2R_BASE_URL")?;
        let r2r_user = std::env::var("R2R_USER")?;
        let r2r_pwd = std::env::var("R2R_PWD")?;
        let r2r_client = Arc::new(HttpR2rClient::new(r2r_base_url.clone(), r2r_user, r2r_pwd));

        let jira_url = std::env::var("JIRA_URL")?;
        let jira_user = std::env::var("JIRA_USERNAME")?;
        let jira_token = std::env::var("JIRA_API_TOKEN")?;
        let jira_client = Arc::new(HttpJiraClient::new(jira_url, jira_user, jira_token));

        let gitlab_url =
            std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".to_string());
        let gitlab_token = std::env::var("GITLAB_API_TOKEN").unwrap_or_else(|_| "".to_string());
        let gitlab_client = Arc::new(HttpGitlabClient::new(gitlab_url, gitlab_token));

        let kafka_brokers = std::env::var("KAFKA_BROKERS")
            .unwrap_or_else(|_| "my-kafka-cluster.confluent.svc.cluster.local:9092".to_string());
        #[cfg(feature = "production")]
        let kafka_client: Arc<dyn KafkaClient> =
            Arc::new(factory_infrastructure::RdKafkaClient::new(&kafka_brokers)?);
        #[cfg(not(feature = "production"))]
        let kafka_client: Arc<dyn KafkaClient> =
            Arc::new(SimpleMockKafkaClient::new(&kafka_brokers).unwrap());

        self.add_tool(Box::new(ExecuteCodeTool::new(sandbox_driver.clone())))
            .await;
        self.add_tool(Box::new(LaunchSandboxPodTool::new())).await;
        self.add_tool(Box::new(PlanMissionTool::new(
            litellm_api_key,
            litellm_base_url,
            planner_model,
            finops_tag,
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
        self.add_tool(Box::new(SecurityReviewTool::with_model(security_model)))
            .await;
        self.add_tool(Box::new(UpdateMissionStatusTool::new("wiki".to_string())))
            .await;
        self.add_tool(Box::new(SpecKitTool::new(specify_cli_path)))
            .await;
        self.add_tool(Box::new(SpecKitTasksToIssuesTool::new(gitlab_client)))
            .await;
        self.add_tool(Box::new(DeepResearchTool::new(kafka_client)))
            .await;
        self.add_tool(Box::new(BridgeTool)).await;
        self.add_tool(Box::new(tools::ListMinioBucketsTool::new()))
            .await;
        self.add_tool(Box::new(tools::ListMinioObjectsTool::new()))
            .await;
        self.add_tool(Box::new(tools::InspectKafkaTopicTool::new()))
            .await;
        self.add_tool(Box::new(tools::GetFactoryStatusTool::new()))
            .await;

        Ok(())
    }

    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        match request.method.as_str() {
            "initialize" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {},
                    "serverInfo": {
                        "name": "factory-mcp-server",
                        "version": "1.0.0"
                    }
                })),
                error: None,
                id: request.id,
            },
            "notifications/initialized" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(serde_json::json!({})),
                error: None,
                id: request.id,
            },
            "ping" => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(serde_json::json!({})),
                error: None,
                id: request.id,
            },
            "list_tools" | "tools/list" => self.handle_list_tools(request.id).await,
            "call_tool" | "tools/call" => self.handle_call_tool(request).await,
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

    pub async fn sse_handler(State(server): State<Arc<McpServer>>, headers: HeaderMap) -> Response {
        server.prune_expired_and_closed_sessions().await;

        let owner_id = server.resolve_owner_id(&headers);
        let session_id = uuid::Uuid::new_v4().to_string();
        // Bounded channel capacity (32 messages) so slow/abandoned clients cannot exhaust memory
        let (tx, rx) = mpsc::channel::<JsonRpcResponse>(32);

        {
            let mut sessions = server.sessions.write().await;
            if sessions.len() >= server.max_sessions {
                return (
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(json!({"error": "Maximum concurrent SSE sessions reached"})),
                )
                    .into_response();
            }
            let now = Instant::now();
            sessions.insert(
                session_id.clone(),
                SessionRecord {
                    owner_id,
                    created_at: now,
                    last_active: now,
                    tx,
                },
            );
        }

        tracing::info!("New SSE session established: {}", session_id);

        let stream = ReceiverStream::new(rx).map(|msg| {
            let json = serde_json::to_string(&msg).unwrap();
            Ok::<Event, Infallible>(Event::default().event("message").data(json))
        });

        let endpoint_url = format!("/mcp?session_id={}", session_id);
        let endpoint_event = Event::default().event("endpoint").data(endpoint_url);

        let initial_stream = tokio_stream::once(Ok(endpoint_event));
        let combined_stream = initial_stream.chain(stream);
        let guarded_stream = SessionDisconnectGuard {
            inner: Box::pin(combined_stream),
            session_id: session_id.clone(),
            sessions: server.sessions.clone(),
        };

        let mut resp = Sse::new(guarded_stream)
            .keep_alive(ax_keep_alive())
            .into_response();
        if let Ok(hv) = HeaderValue::from_str(&session_id) {
            resp.headers_mut().insert("x-mcp-session-id", hv);
        }
        resp
    }

    pub async fn post_handler(
        State(server): State<Arc<McpServer>>,
        headers: HeaderMap,
        Query(params): Query<HashMap<String, String>>,
        Json(request): Json<JsonRpcRequest>,
    ) -> Response {
        server.prune_expired_and_closed_sessions().await;

        let caller_owner = server.resolve_owner_id(&headers);
        let session_id = params.get("session_id");

        if let Some(sid) = session_id {
            let mut sessions = server.sessions.write().await;
            let Some(rec) = sessions.get_mut(sid) else {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "SSE session not found or expired"})),
                )
                    .into_response();
            };
            if rec.owner_id != caller_owner {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Caller does not own the requested SSE session"})),
                )
                    .into_response();
            }
            rec.last_active = Instant::now();
        }

        let response = server.handle_request(request).await;

        if let Some(sid) = session_id {
            let sessions = server.sessions.read().await;
            if let Some(rec) = sessions.get(sid) {
                let _ = rec.tx.try_send(response.clone());
            }
        }

        Json(response).into_response()
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

async fn require_privileged_auth(
    State(server): State<Arc<McpServer>>,
    req: Request,
    next: Next,
) -> Response {
    if !server.verify_caller_authorized(req.headers()) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Unauthorized: missing or invalid bearer authentication for privileged route"})),
        )
            .into_response();
    }
    next.run(req).await
}

pub fn build_mcp_router(server: Arc<McpServer>) -> Router {
    let privileged = Router::new()
        .route("/mcp", post(McpServer::post_handler))
        .route("/sse", get(McpServer::sse_handler))
        .route("/feedback", post(crate::feedback_route::handle_feedback))
        .route_layer(middleware::from_fn_with_state(
            server.clone(),
            require_privileged_auth,
        ));

    Router::new()
        .merge(privileged)
        .route(
            "/webhooks/github",
            post(crate::github_webhook::handle_github_webhook),
        )
        .route("/health", get(|| async { Json(json!({"status": "ok"})) }))
        .with_state(server)
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
        mock_tool
            .expect_call()
            .returning(|_| Err(anyhow::anyhow!("Sensitive internal error details")));

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
