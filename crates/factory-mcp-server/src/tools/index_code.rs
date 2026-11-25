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

        let mut form = reqwest::multipart::Form::new().text("raw_text", content.to_string());
        if !metadata.is_null() {
            form = form.text("metadata", serde_json::to_string(&metadata)?);
        }

        let response = self
            .http_client
            .post(format!(
                "{}/v3/documents",
                self.r2r_base_url.trim_end_matches('/')
            ))
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

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: "Document indexed successfully".to_string(),
            }],
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Bytes, extract::State, http::HeaderMap, routing::post, Router};
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
        type Captured = Arc<Mutex<Option<(String, String)>>>;
        let captured: Captured = Arc::new(Mutex::new(None));

        let app = Router::new()
            .route(
                "/v3/documents",
                post(
                    |State(state): State<Captured>, headers: HeaderMap, body: Bytes| async move {
                        let ct = headers
                            .get(axum::http::header::CONTENT_TYPE)
                            .and_then(|v| v.to_str().ok())
                            .unwrap_or_default()
                            .to_string();
                        let body_str = String::from_utf8_lossy(&body).to_string();
                        *state.lock().await = Some((ct, body_str));
                        axum::http::StatusCode::ACCEPTED
                    },
                ),
            )
            .with_state(captured.clone());

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

        let recorded = captured.lock().await.clone().expect("request captured");
        assert!(
            recorded.0.starts_with("multipart/form-data"),
            "expected multipart/form-data, got {}",
            recorded.0
        );
        assert!(recorded.1.contains("name=\"raw_text\""));
        assert!(recorded.1.contains("# Demo\nfn main() {}"));
        assert!(recorded.1.contains("name=\"metadata\""));
        assert!(recorded.1.contains("\"repository\":\"lgcorzo/demo\""));
        assert!(recorded.1.contains("\"title\":\"lgcorzo_demo_abcdef12\""));

        handle.abort();
    }

    #[tokio::test]
    async fn test_index_code_tool_treats_409_conflict_as_idempotent_success() {
        let app = Router::new().route(
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
