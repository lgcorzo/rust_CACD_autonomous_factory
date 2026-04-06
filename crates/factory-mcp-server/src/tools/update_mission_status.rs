use crate::protocol::{CallToolResult, McpContent};
use crate::tools::Tool;
use async_trait::async_trait;
use chrono::Local;
use serde_json::{json, Value};
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct UpdateMissionStatusTool {
    docs_path: String,
}

impl UpdateMissionStatusTool {
    pub fn new(docs_path: String) -> Self {
        Self { docs_path }
    }
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

        // Validate mission_id to prevent path traversal
        if mission_id.contains('/') || mission_id.contains('\\') || mission_id.contains("..") {
            return Ok(CallToolResult {
                content: vec![McpContent::Text {
                    text: format!("Invalid mission_id: {}. It cannot contain path separators or '..'.", mission_id),
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

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // 1. Update wiki/mission_history.md (Terminal-friendly table)
        let history_path = format!("{}/mission_history.md", self.docs_path);
        let mut file = OpenOptions::new().append(true).open(&history_path)?;

        let new_entry = format!(
            "| {} | {} | {} | {} | {} | (Agent: {}) |\n",
            mission_id, timestamp, status, summary, artifacts, agent_name
        );
        file.write_all(new_entry.as_bytes())?;

        // 2. Update a detailed summary file for the specific mission
        let summary_path = format!("{}/mission_{}_summary.md", self.docs_path, mission_id);
        let mut summary_file = File::create(&summary_path)?;
        let summary_content = format!(
            "# Mission Summary: {}\n\n- **Status**: {}\n- **Date**: {}\n- **Agent**: {}\n\n## Summary\n{}\n\n## Artifacts\n{}\n",
            mission_id, status, timestamp, agent_name, summary, artifacts
        );
        summary_file.write_all(summary_content.as_bytes())?;

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
    use serde_json::json;
    use std::fs;

    #[tokio::test]
    async fn test_update_mission_status_vulnerability() {
        let temp_dir = std::env::temp_dir().join(uuid::Uuid::new_v4().to_string());
        let docs_path_buf = temp_dir.join("docs");
        fs::create_dir_all(&docs_path_buf).unwrap();

        // Create a directory that allows us to go up
        fs::create_dir_all(docs_path_buf.join("mission_foo")).unwrap();

        let docs_path = docs_path_buf.to_str().unwrap().to_string();
        let history_path = docs_path_buf.join("mission_history.md");
        fs::write(&history_path, "").unwrap();

        let tool = UpdateMissionStatusTool::new(docs_path.clone());

        // This mission_id attempts to create a file outside the docs_path
        let malicious_mission_id = "foo/../../pwned";
        let params = json!({
            "mission_id": malicious_mission_id,
            "status": "Success",
            "summary": "Evil summary",
            "agent_name": "Attacker"
        });

        let result = tool.call(params).await;

        // Now, this should return an error
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_error);
        if let McpContent::Text { text } = &result.content[0] {
            assert!(text.contains("Invalid mission_id"));
        } else {
            panic!("Expected text content");
        }

        // Check that the file was NOT created
        let pwned_file = temp_dir.join("pwned_summary.md");
        assert!(!pwned_file.exists());

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[tokio::test]
    async fn test_update_mission_status_valid_id() {
        let temp_dir = std::env::temp_dir().join(uuid::Uuid::new_v4().to_string());
        let docs_path_buf = temp_dir.join("docs");
        fs::create_dir_all(&docs_path_buf).unwrap();

        let docs_path = docs_path_buf.to_str().unwrap().to_string();
        let history_path = docs_path_buf.join("mission_history.md");
        fs::write(&history_path, "").unwrap();

        let tool = UpdateMissionStatusTool::new(docs_path.clone());

        let valid_mission_id = "mission-123_abc";
        let params = json!({
            "mission_id": valid_mission_id,
            "status": "Success",
            "summary": "Valid summary"
        });

        let result = tool.call(params).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_error);

        let summary_file = docs_path_buf.join(format!("mission_{}_summary.md", valid_mission_id));
        assert!(summary_file.exists());

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
