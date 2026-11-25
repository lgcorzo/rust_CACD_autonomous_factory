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

    async fn resolve_or_create_collection(&self, name: &str) -> anyhow::Result<String> {
        let base = self.r2r_base_url.trim_end_matches('/');
        let get_url = format!("{}/v3/collections/name/{}", base, name);
        let get_res = self.http_client.get(&get_url).send().await?;

        if get_res.status().is_success() {
            let v: Value = get_res.json().await?;
            return v["results"]["id"]
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| anyhow::anyhow!("Missing collection id in R2R response"));
        }

        if get_res.status() != reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!("Unexpected R2R GET collection status: {}", get_res.status());
        }

        let create_url = format!("{}/v3/collections", base);
        let create_res = self
            .http_client
            .post(&create_url)
            .json(&json!({
                "name": name,
                "description": format!("Autonomous Factory AST & Knowledge Graph collection for {}", name)
            }))
            .send()
            .await?;

        if !create_res.status().is_success() {
            anyhow::bail!(
                "Failed to create R2R collection '{}': {}",
                name,
                create_res.status()
            );
        }

        let v: Value = create_res.json().await?;
        v["results"]["id"]
            .as_str()
            .map(str::to_string)
            .ok_or_else(|| anyhow::anyhow!("Missing created collection id in R2R response"))
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
                "metadata": {"type": "object"},
                "collection_name": {"type": "string"},
                "run_with_orchestration": {"type": "boolean"}
            },
            "required": ["content"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let content = params["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Content is required"))?;

        let mut metadata = params["metadata"].clone();
        if let Some(obj) = metadata.as_object_mut() {
            let derived_title = (!obj.contains_key("title"))
                .then(|| {
                    let repo = obj.get("repository").and_then(Value::as_str)?;
                    let sha = obj.get("commit_sha").and_then(Value::as_str)?;
                    let short_sha: String = sha.chars().take(8).collect();
                    Some(format!("{}_{}", repo.replace('/', "_"), short_sha))
                })
                .flatten();
            if let Some(title) = derived_title {
                obj.insert("title".to_string(), Value::String(title));
            }
        }

        let collection_name = params["collection_name"]
            .as_str()
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .or_else(|| {
                metadata
                    .get("repository")
                    .and_then(Value::as_str)
                    .filter(|r| !r.is_empty())
                    .map(|r| r.replace('/', "-"))
            });

        let collection_id = if let Some(col_name) = collection_name.as_deref() {
            match self.resolve_or_create_collection(col_name).await {
                Ok(cid) => Some(cid),
                Err(e) => {
                    tracing::warn!("Failed to resolve R2R collection '{}': {}", col_name, e);
                    None
                }
            }
        } else {
            None
        };

        if let (Some(cid), Some(obj)) = (collection_id.as_deref(), metadata.as_object_mut()) {
            obj.insert("collection_id".to_string(), Value::String(cid.to_string()));
        }

        let run_with_orchestration = params["run_with_orchestration"].as_bool().unwrap_or(true);
        let mut form = reqwest::multipart::Form::new()
            .text("raw_text", content.to_string())
            .text(
                "run_with_orchestration",
                if run_with_orchestration {
                    "true"
                } else {
                    "false"
                },
            );
        if !metadata.is_null() {
            form = form.text("metadata", serde_json::to_string(&metadata)?);
        }
        if let Some(cid) = collection_id.as_deref() {
            form = form.text("collection_ids", serde_json::to_string(&vec![cid])?);
        }

        let base = self.r2r_base_url.trim_end_matches('/');
        let response = self
            .http_client
            .post(format!("{}/v3/documents", base))
            .multipart(form)
            .send()
            .await?;

        let status = response.status();
        if status == reqwest::StatusCode::CONFLICT {
            return Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: "Document already indexed (idempotent 409)".to_string(),
                }],
                is_error: false,
            });
        }

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: format!("R2R API error: {} - {}", status, body),
                }],
                is_error: true,
            });
        }

        let body_text = response.text().await.unwrap_or_default();
        let parsed_body: Value = serde_json::from_str(&body_text).unwrap_or(Value::Null);
        let document_id = parsed_body["results"]["document_id"]
            .as_str()
            .map(str::to_string);
        let task_id = parsed_body["results"]["task_id"]
            .as_str()
            .map(str::to_string);

        if let (Some(cid), Some(did)) = (collection_id.as_deref(), document_id.as_deref()) {
            let assign_url = format!("{}/v3/collections/{}/documents/{}", base, cid, did);
            if let Err(e) = self.http_client.post(&assign_url).send().await {
                tracing::warn!(
                    "Failed best-effort collection assignment for doc {} -> {}: {}",
                    did,
                    cid,
                    e
                );
            }
        }

        let summary = json!({
            "message": "Document indexed successfully",
            "document_id": document_id,
            "task_id": task_id,
            "collection_id": collection_id,
        });

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: summary.to_string(),
            }],
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Bytes,
        extract::State,
        http::HeaderMap,
        routing::{get, post},
        Json, Router,
    };
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_index_code_tool_missing_content() {
        let tool = IndexCodeTool::new("http://localhost:8000".to_string());
        let params = json!({});
        let result = tool.call(params).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Content is required");
    }

    #[tokio::test]
    async fn test_index_code_tool_sends_multipart_to_v3_documents() {
        #[derive(Clone, Default)]
        struct TestState {
            doc_req: Arc<Mutex<Option<(String, String)>>>,
            assigned: Arc<Mutex<bool>>,
            created_col: Arc<Mutex<bool>>,
        }

        let state = TestState::default();
        let cid = "11111111-1111-1111-1111-111111111111";
        let did = "22222222-2222-2222-2222-222222222222";

        let app = Router::new()
            .route(
                "/v3/collections/name/lgcorzo-demo",
                get(|| async { axum::http::StatusCode::NOT_FOUND }),
            )
            .route(
                "/v3/collections",
                post(move |State(st): State<TestState>| async move {
                    *st.created_col.lock().await = true;
                    Json(json!({ "results": { "id": cid } }))
                }),
            )
            .route(
                "/v3/collections/11111111-1111-1111-1111-111111111111/documents/22222222-2222-2222-2222-222222222222",
                post(|State(st): State<TestState>| async move {
                    *st.assigned.lock().await = true;
                    Json(json!({ "results": { "success": true } }))
                }),
            )
            .route(
                "/v3/documents",
                post(
                    move |State(st): State<TestState>, headers: HeaderMap, body: Bytes| async move {
                        let ct = headers
                            .get(axum::http::header::CONTENT_TYPE)
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or_default()
                            .to_string();
                        let body_str = String::from_utf8_lossy(&body).to_string();
                        *st.doc_req.lock().await = Some((ct, body_str));
                        (
                            axum::http::StatusCode::ACCEPTED,
                            Json(json!({
                                "results": {
                                    "document_id": did,
                                    "task_id": "hatchet-task-123"
                                }
                            })),
                        )
                    },
                ),
            )
            .with_state(state.clone());

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        let tool = IndexCodeTool::new(format!("http://{}", addr));
        let params = json!({
            "content": "# Demo\nfn main() {}",
            "metadata": {
                "repository": "lgcorzo/demo",
                "branch": "main",
                "commit_sha": "abcdef1234567890",
                "verified": "true"
            }
        });

        let result = tool.call(params).await.unwrap();
        assert!(!result.is_error);
        assert!(matches!(
            &result.content[0],
            McpContent::Text { text } if text.contains("hatchet-task-123") && text.contains(cid)
        ));
        assert!(*state.created_col.lock().await);
        assert!(*state.assigned.lock().await);

        let recorded = state
            .doc_req
            .lock()
            .await
            .clone()
            .expect("request captured");
        assert!(
            recorded.0.starts_with("multipart/form-data"),
            "expected multipart/form-data, got {}",
            recorded.0
        );
        assert!(recorded.1.contains("name=\"raw_text\""));
        assert!(recorded.1.contains("# Demo\nfn main() {}"));
        assert!(recorded.1.contains("name=\"run_with_orchestration\""));
        assert!(recorded.1.contains("true"));
        assert!(recorded.1.contains("name=\"collection_ids\""));
        assert!(recorded.1.contains(cid));
        assert!(recorded.1.contains("name=\"metadata\""));
        assert!(recorded.1.contains("\"repository\":\"lgcorzo/demo\""));
        assert!(recorded.1.contains("\"title\":\"lgcorzo_demo_abcdef12\""));
        assert!(recorded
            .1
            .contains(&format!("\"collection_id\":\"{}\"", cid)));

        handle.abort();
    }

    #[tokio::test]
    async fn test_index_code_tool_treats_409_conflict_as_idempotent_success() {
        let app = Router::new()
            .route(
                "/v3/collections/name/lgcorzo-demo",
                get(|| async { axum::http::StatusCode::NOT_FOUND }),
            )
            .route(
                "/v3/documents",
                post(|| async { axum::http::StatusCode::CONFLICT }),
            );

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        let tool = IndexCodeTool::new(format!("http://{}", addr));
        let params = json!({
            "content": "duplicate content",
            "metadata": {
                "repository": "lgcorzo/demo",
                "commit_sha": "abcdef12"
            }
        });

        let result = tool.call(params).await.unwrap();
        assert!(!result.is_error);
        assert!(matches!(
            &result.content[0],
            McpContent::Text { text } if text.contains("already indexed")
        ));

        handle.abort();
    }
}
