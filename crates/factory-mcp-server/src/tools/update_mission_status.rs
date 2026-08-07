use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use chrono::Local;
use serde_json::{json, Value};
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

pub struct UpdateMissionStatusTool {
    docs_path: String,
}

impl UpdateMissionStatusTool {
    pub fn new(docs_path: String) -> Self {
        Self { docs_path }
    }
}

fn sanitize_markdown_table_field(s: &str) -> String {
    s.replace('\r', "").replace('\n', " ").replace('|', "\\|")
}

#[async_trait]
impl Tool for UpdateMissionStatusTool {
    fn name(&self) -> String {
        "update_mission_status".to_string()
    }

    fn description(&self) -> String {
        "Updates project documentation and mission history after a mission or testing phase."
            .to_string()
    }

    fn input_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "mission_id": {"type": "string"},
                "status": {"type": "string", "enum": ["Success", "Failure", "In Progress"]},
                "summary": {"type": "string"},
                "artifacts": {"type": "array", "items": {"type": "string"}},
                "agent_name": {"type": "string"}
            },
            "required": ["mission_id", "status", "summary"]
        })
    }

    async fn call(&self, params: Value) -> anyhow::Result<CallToolResult> {
        let mission_id = params["mission_id"].as_str().unwrap_or("unknown");

        // Security check: prevent path traversal
        if mission_id.contains('/') || mission_id.contains('\\') || mission_id.contains("..") {
            return Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: "Error: Invalid mission_id. Path traversal characters are not allowed."
                        .to_string(),
                }],
                is_error: true,
            });
        }

        let status = params["status"].as_str().unwrap_or("Unknown");
        let summary = params["summary"].as_str().unwrap_or("");
        let agent_name = params["agent_name"].as_str().unwrap_or("OpenCodeAgent");
        let artifacts = params["artifacts"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|v| v.as_str().unwrap_or(""))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();

        // Security check: prevent markdown table injection & newline attacks
        let status_sanitized = sanitize_markdown_table_field(status);
        let summary_sanitized = sanitize_markdown_table_field(summary);
        let agent_name_sanitized = sanitize_markdown_table_field(agent_name);
        let artifacts_sanitized = sanitize_markdown_table_field(&artifacts);

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // 1. Update wiki/mission_history.md (Terminal-friendly table)
        let history_path = format!("{}/mission_history.md", self.docs_path);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&history_path)
            .await?;

        let new_entry = format!(
            "| {} | {} | {} | {} | {} | (Agent: {}) |\n",
            mission_id,
            timestamp,
            status_sanitized,
            summary_sanitized,
            artifacts_sanitized,
            agent_name_sanitized
        );
        file.write_all(new_entry.as_bytes()).await?;

        // 2. Update a detailed summary file for the specific mission
        let summary_path = format!("{}/mission_{}_summary.md", self.docs_path, mission_id);
        let mut summary_file = File::create(&summary_path).await?;
        let summary_content = format!(
            "# Mission Summary: {}\n\n- **Status**: {}\n- **Date**: {}\n- **Agent**: {}\n\n## Summary\n{}\n\n## Artifacts\n{}\n",
            mission_id,
            status_sanitized,
            timestamp,
            agent_name_sanitized,
            summary_sanitized,
            artifacts_sanitized
        );
        summary_file.write_all(summary_content.as_bytes()).await?;

        Ok(CallToolResult {
            content: vec![McpContent::Text {
                text: format!("Mission {} updated in documentation.", mission_id),
            }],
            is_error: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_mission_status_success_and_sanitization() {
        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push(format!("test-mission-status-{}", uuid::Uuid::new_v4()));
        tokio::fs::create_dir_all(&tmp_dir).await.unwrap();

        let docs_path = tmp_dir.to_string_lossy().into_owned();
        let history_file_path = tmp_dir.join("mission_history.md");

        let tool = UpdateMissionStatusTool::new(docs_path.clone());

        // Malicious inputs containing newlines and pipes
        let params = json!({
            "mission_id": "test_mission_123",
            "status": "In\nProgress",
            "summary": "This is a summary | with pipe \r\n and newlines.",
            "agent_name": "Agent | 007",
            "artifacts": ["artifact1\nwith_newline", "artifact2|with_pipe"]
        });

        let result = tool.call(params).await.unwrap();
        assert!(!result.is_error);

        // Verify mission_history.md content is sanitized
        let history_content = tokio::fs::read_to_string(&history_file_path).await.unwrap();
        assert!(history_content.contains("In Progress"));
        assert!(history_content.contains("This is a summary \\| with pipe   and newlines."));
        assert!(history_content.contains("Agent \\| 007"));
        assert!(history_content.contains("artifact1 with_newline, artifact2\\|with_pipe"));

        // Verify detailed summary file is formatted and sanitized as well
        let summary_file_path = tmp_dir.join("mission_test_mission_123_summary.md");
        let summary_content = tokio::fs::read_to_string(&summary_file_path).await.unwrap();
        assert!(summary_content.contains("# Mission Summary: test_mission_123"));
        assert!(summary_content.contains("- **Status**: In Progress"));

        // Clean up
        let _ = tokio::fs::remove_dir_all(&tmp_dir).await;
    }

    #[tokio::test]
    async fn test_update_mission_status_path_traversal_rejected() {
        let tool = UpdateMissionStatusTool::new("wiki".to_string());

        let params = json!({
            "mission_id": "../malicious",
            "status": "Success",
            "summary": "Attempting path traversal"
        });

        let result = tool.call(params).await.unwrap();
        assert!(result.is_error);
        if let McpContent::Text { text } = &result.content[0] {
            assert!(text
                .contains("Error: Invalid mission_id. Path traversal characters are not allowed."));
        }
    }
}
